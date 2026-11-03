# The correction to the Hetzner note (Mustafa, 2026-11-03, received about 18:32 Amsterdam, in the session's chat; word for word)

Correction to my Hetzner note (Mustafa, 2026-11-03): the game server is a CCX23 for development and "does it work" runs. The CCX23 (dedicated) is used only for the measurement runs: tick timing, the 60 Hz margin and rooms per VM. Each run's plan says which server it needs.

## The screenshot he attached (the session's description, not his words)
Hetzner Cloud's "Create a server" page. Type: three groups. Shared Resources: "Cost-Optimized" (badge "Limited
availability"; "Cost-efficient on older hardware generations, with limited availability."; architecture "x86
(Intel®/AMD)" or "Arm64 (Ampere®)", neither selected) and "Regular Performance" ("Higher CPU performance based on newer
hardware generations."; architecture "x86 (AMD)"). Dedicated Resources: "General Purpose" (selected; "Provides dedicated
vCPUs on the latest available hardware generation at the location."; architecture "x86 (AMD)"). Its table: CCX13 2 vCPU
8 GB 80 GB 20 TB €0.083/h €52.02/mo; CCX23 4 vCPU 16 GB 160 GB 20 TB €0.167/h €104.05/mo (selected); CCX33 8 vCPU
32 GB 240 GB 30 TB €0.268/h €167.57/mo; CCX43 16 vCPU 64 GB 360 GB 40 TB €0.535/h €333.95/mo; CCX53 32 vCPU 128 GB
600 GB 50 TB €1.035/h €645.52/mo; CCX63 48 vCPU 192 GB 960 GB 60 TB €1.655/h €1,032.72/mo. The order summary on the
right: "CCX23 Dedicated CPU" (Type), "Nuremberg" (Location), "Ubuntu 26.04" (Image), "IPv4, IPv6" (Networking), SSH keys
not yet set; "1 SERVER €104.05/mo", "1 IPV4 €0.61/mo", "TOTAL €104.65/mo". The page states no VAT; the research's
CCX23 price is 86.49 EUR a month excluding VAT, IPv4 included (R6-HOST-F15), and 86.49 × 1.21 = 104.65 (the session's
arithmetic; the VAT rate is not stated on the page).
