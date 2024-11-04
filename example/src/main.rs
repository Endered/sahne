use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use sahne::mixin;
use sahne::provider;

trait UseCurrentTime {
    fn current_time() -> SystemTime;
}

#[provider(UseCurrentTime)]
trait ProvideCurrentTime {
    fn current_time() -> SystemTime {
        SystemTime::now()
    }
}

#[provider(UseCurrentTime)]
trait DummyCurrentTime {
    fn current_time() -> SystemTime {
        UNIX_EPOCH
    }
}

trait Alert: UseCurrentTime {
    fn alert() {
        println!(
            "Current time is {}!!",
            Self::current_time()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs()
        );
    }
}

#[mixin(ProvideCurrentTime)]
struct AlertImpl {}
impl Alert for AlertImpl {}

#[mixin(DummyCurrentTime)]
struct AlertDummyImpl {}
impl Alert for AlertDummyImpl {}

trait MutationTest {
    fn f();
    fn f_ref(&self);
    fn f_mut(&mut self);
}

#[provider(MutationTest)]
trait MutationTestProvider {
    fn f() {
        todo!()
    }
    fn f_ref(&self) {
        todo!()
    }
    fn f_mut(&mut self) {
        todo!()
    }
}

#[mixin(MutationTestProvider)]
struct MutationTestImpl {}

fn main() {
    AlertImpl::alert();

    AlertDummyImpl::alert();
}
