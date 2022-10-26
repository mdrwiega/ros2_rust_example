# ros2_rust_example

Example of ROS 2 package written in Rust.
This repository is a part of the [tutorial](https://mdrwiega.com/ros2-with-rust/) published on by blog page.

## Rust support in ROS 2

Rust is not supported in ROS 2 by default. However, there are two solutions to provide such a support.
- [ros2-rust](https://github.com/ros2-rust/ros2_rust)
- [r2r](https://github.com/sequenceplanner/r2r)

I have used the first one (ros2-rust) package. Mostly, because it's more actively developed and in my opinion has a better documentation. So, the presented example will be based on this approach.

## Prerequisite

### Testing Environment

I have used the following specification. However, newer versions should be also fine, especially Ubuntu 22.04 and ROS 2 Humble.

Used software:
- Ubuntu 20.04
- Python 3.9
- rustc 1.64.0 (Rust compiler)
- ROS 2 Galactic

#### Environment preparation

- Install Python 3.8+
- Install ROS 2 Galactic (or Humble). [Here is the instruction for Ubuntu](https://docs.ros.org/en/galactic/Installation.html).
- Install Rust (the script is intended for Linux).
```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

- If you have already installed the Rust, then just update it:
```bash
rustup update
```

- Install tools necessary to configure and build the `ros2-rust` ROS package
```bash
sudo apt install -y git libclang-dev python3-vcstool python3-pip
```

- Install packages necessary to use `Cargo` with ROS packaging system `colcon`
```bash
cargo install cargo-ament-build
pip install git+https://github.com/colcon/colcon-cargo.git
pip install git+https://github.com/colcon/colcon-ros-cargo.git
```

### Workspace creation

- Create a new workspace
```bash
mkdir -p ~/ros2_rust_ws/src
cd ~/ros2_rust_ws/src
```

### Build `ros2_rust` from sources

- Clone the [ros2-rust](https://github.com/ros2-rust/ros2_rust) repository to the workspace
```bash
cd ~/ros2_rust_ws/src
git clone git@github.com:ros2-rust/ros2_rust.git
```

- Get all `ros2-rust` dependencies and source ROS configuration. Make sure that a `ROS_DISTRO` is set to the correct ROS version (currently `ros2_rust` supports foxy, galactic, humble and rolling)
```bash
cd ..
vcs import src < src/ros2_rust/ros2_rust_${ROS_DISTRO}.repos
. /opt/ros/${ROS_DISTRO}/setup.sh
```

- Build the workspace with `ros2-rust`
```bash
colcon build --packages-up-to examples_rclrs_minimal_pub_sub
```
In case of building issues please check [the original building instruction](https://github.com/ros2-rust/ros2_rust/blob/main/docs/building.md).

## Build and run this package

- Clone repository to workspace
```bash
cd ~/ros2_rust_ws/src
git clone git@github.com:mdrwiega/ros2_rust_example.git
```

- Build the package
```bash
colcon build --packages-select rust_example
```

- Run publisher and subscriber. Two terminal will be necessary. Run the publisher node in the first terminal
```bash
ros2 run rust_example publisher
```

- In the second terminal run the subscriber node
```bash
ros2 run rust_example subscriber
```

- Alternatively, run the nodes with created launch file
```bash
ros2 launch rust_example publisher_and_subscriber_launch.yaml
```
