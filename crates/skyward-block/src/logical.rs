//! Single-slot dynamic partitions, including Mi-Thorium's retrofit layout.

use crate::{boot_param, devnum, dm, BY_NAME};
use anyhow::{ensure, Context, Result};
use skyward_lp::{Extent, Metadata, Partition, ATTR_DISABLED};
use std::path::{Path, PathBuf};

fn targets(partition: &Partition, devices: &[(u32, u32)]) -> Result<Vec<dm::Target>> {
    partition
        .extents
        .iter()
        .map(|extent| {
            Ok(match *extent {
                Extent::Linear { sectors, device, sector } => {
                    dm::Target::Linear { sectors, device: *devices.get(device).context("logical extent names an absent block device")?, offset: sector }
                }
                Extent::Zero { sectors } => dm::Target::Zero { sectors },
            })
        })
        .collect()
}

/// Reach a logical partition through device-mapper. Metadata is in
/// `androidboot.super_partition` (`system` on mi8937), and each extent
/// resolves against its declared backing device, including `cust`.
pub fn device(name: &str) -> Result<PathBuf> {
    ensure!(!name.is_empty() && !name.contains('/'), "invalid logical partition name {name:?}");
    if let Some((_, minor)) = dm::status(name)? {
        return Ok(PathBuf::from(format!("/dev/block/dm-{minor}")));
    }
    let super_name = boot_param("androidboot.super_partition")?.unwrap_or_else(|| "super".into());
    ensure!(!super_name.contains('/'), "invalid super partition name {super_name:?}");
    let super_device = Path::new(BY_NAME).join(super_name);
    let metadata: Metadata = skyward_lp::read(&super_device, 0)?;
    let partition = metadata.partitions.iter().find(|p| p.name == name).with_context(|| format!("super contains no logical partition {name}"))?;
    ensure!(partition.attributes & ATTR_DISABLED == 0 && !partition.extents.is_empty(), "logical partition {name} is disabled or empty");
    let devices = metadata
        .block_devices
        .iter()
        .enumerate()
        .map(|(i, d)| if i == 0 { devnum(&super_device) } else { devnum(Path::new(BY_NAME).join(&d.partition_name)) })
        .collect::<Result<Vec<_>>>()?;
    dm::create(name, &targets(partition, &devices)?, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn retrofit_extents_resolve_the_backing_device_index() {
        let partition = Partition {
            name: "system".into(),
            attributes: 0,
            group: 0,
            extents: vec![Extent::Linear { sectors: 8, device: 1, sector: 4096 }, Extent::Zero { sectors: 2 }],
        };
        let mapped = targets(&partition, &[(179, 1), (179, 2)]).unwrap();
        assert_eq!(mapped[0], dm::Target::Linear { sectors: 8, device: (179, 2), offset: 4096 });
        assert_eq!(mapped[1], dm::Target::Zero { sectors: 2 });
        assert!(targets(&partition, &[(179, 1)]).is_err());
    }
}
