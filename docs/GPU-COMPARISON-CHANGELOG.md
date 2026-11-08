# GPU Comparison engineering log

## 2026-11-08

Base: nokyan/resources `45b4c122d5acd29788934469fd123ae62296bea2`, version 1.10.2. Fork: Cyclotron555/resources. Feature branch: `feature/gpu-comparison`. The upstream GPL-3.0-or-later license and credits are preserved. Upstream development moved to GNOME GitLab; this branch intentionally starts from the GitHub 1.10.2 snapshot matching the locally installed version.

### Changes

- Add a GPU Comparison sidebar page when at least two GPUs are detected. Equal-width columns reuse existing ResGPU widgets and sensor formatting. Model names and PCI identifiers remain above each column. Each column scrolls vertically; the overall view scrolls horizontally on smaller windows or for more cards.
- The comparison hides media-engine graphs and frequency rows to keep GPU load, VRAM, power and temperature visible together. Original individual pages keep the full layout. Temperature auto-scaling and all upstream sensor semantics remain unchanged.
- Reuse the same polled GpuData sample for the original and comparison views; no new polling thread, NVML command or driver setting. Match refreshes by GpuIdentifier rather than pairing HashMap iteration order with a vector. Individual page sort order starts after the comparison page.
- Register the new translatable strings in POTFILES.in; existing translations remain unchanged.
- Add a separate resources-capture documentation helper that renders only this app's own GTK window into a PNG. It opens a 1550x1100 window, waits ten seconds for live readings, saves and exits. It does not capture other desktop windows. No fake graphs or generated screenshots.
- Add build/run scripts, help, two real screenshots and an explanatory README. Local installation uses the upstream development app ID and a separate prefix. The packaged /usr/bin/resources is untouched.

### Validation

Built with workspace-local Ubuntu build packages (Rust 1.93.1, GTK 4.22.4, libadwaita 1.9.1). No system package install, root authentication, driver change or display/power change was required. The first local link attempt needed the extracted development-library symlinks connected to already-installed runtime libraries. A missing rustdoc wrapper initially prevented doctests; fixed only in the local toolset, then repeated the full suite.

All five Meson checks passed: desktop metadata, app metadata, settings schema, main Rust suite (83 unit tests), and process-data Rust suite (14 unit tests plus 3 doctests). New UI behavior was exercised by launching the compiled app and inspecting real exports of the comparison and original RTX page. No automated GPU hotplug or single-GPU integration test was performed.

Hardware verified in the comparison capture: NVIDIA GeForce GTX 1660 at 0000:05:00.0, RTX 3060 at 0000:06:00.0. Both model headings and PCI labels matched their sensor panels, temperatures, memory capacities and individual pages. Normal GPU pages remain accessible. Screen captures show live desktop/background activity, not a controlled GPU stress test. An existing KDE/libadwaita dark-theme warning appeared without preventing rendering or capture.

The separate Wan comparison finished before UI validation: RAM sampling 147.726s versus GTX 163.130s at 512x288, 33 frames, 20 steps; resulting MP4 hashes matched. Those measurements belong to the ComfyUI-MultiGPU log and do not validate Resources' sensor accuracy. No Wan models or private desktop/app/process data are included in this repository.

### Limits and rollback

Visual testing covers this two-NVIDIA-card machine. The new layout follows detected-device order; headings provide stable identity without assuming GPU 1 or cuda:0 is the primary compute card. Per-card temperature charts auto-scale independently, so numeric temperature readings should be compared. More than two columns and narrow windows are supported by horizontal scrolling but have not been extensively tested. Hotplug handling is unchanged. The comparison's sidebar has no aggregate utilization graph, since averaging unrelated GPUs could mislead.

Close and delete the separate local build to roll back; the distribution Resources app remains installed. Keep main/upstream unchanged and submit upstream only after separate review. Publication includes source, build scripts, help, screenshots, original license and this log; no binary portability claim is made.
