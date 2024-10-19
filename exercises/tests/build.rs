//! This is the build script for both tests7 and tests8.
//!
//! You should modify this file to make both exercises pass.

fn main() {
    // In tests7, we should set up an environment variable
    // called `TEST_FOO`. Print in the standard output to let
    // Cargo do it.
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // What's the use of this timestamp here?
      // 设置环境变量 `TEST_FOO`，其值为当前的 UNIX 时间戳
    // 这样在测试中就可以使用这个环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 启用 `pass` 特性，这样在 tests8 中就可以使用这个特性
    // 来让测试用例提前返回
    println!("cargo:rustc-cfg=feature=\"pass\"");
}

