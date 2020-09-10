#![feature(staged_api)]

#[stable(feature = "x", since = "1")]
struct StableType;

#[unstable(feature = "x", issue = "none")]
struct UnstableType;

#[stable(feature = "x", since = "1")]
trait StableTrait {}

#[unstable(feature = "x", issue = "none")]
trait UnstableTrait {}

#[unstable(feature = "x", issue = "none")]
impl UnstableTrait for UnstableType {}

#[unstable(feature = "x", issue = "none")]
impl StableTrait for UnstableType {}

#[unstable(feature = "x", issue = "none")]
impl UnstableTrait for StableType {}

#[unstable(feature = "x", issue = "none")]
//~^ ERROR An `#[unstable]` annotation here has no effect.
impl StableTrait for StableType {}

fn main() {}
