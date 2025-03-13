```bash
sam@bb ~/workspace/kvme $ cargo run -p kv
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.03s
     Running `target/debug/kv`
Initializing VFIO-based NVMe communication...
VFIO API version: 0
VFIO supports TYPE1v2 IOMMU
VFIO group is viable
VFIO group associated with container
IOMMU set to TYPE1
Obtained VFIO device FD: 5
Region Info: size = 0x4000, offset = 0x0
Mapped BAR region at: 0x7f9a7d522000
NVMe Controller Capabilities (CAP): 0x000000203c013fff
NVMe Controller Version (VS): 0x00010400
Unmapped BAR region
```

The nvme spec version in this example is 1.4
`NVMe Controller Version (VS): 0x00010400`
