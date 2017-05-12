

mod model {

    pub trait Model {
        fn mutate(&mut self) {
            unimplemented!();
        }
    }
}

mod source {
    pub trait Source {
        fn get(& self) {
            unimplemented!();
        }
    }
}

mod config {
    use super::model::Model;
    use super::source::Source;
    #[derive(Clone, Copy, Debug)]
    pub struct None;
    impl Model for None {}
    impl Source for None {}

    #[derive(Debug)]
    pub struct Config<M: Model, S: Source> {
        pub model: M,
        pub source: S,
    }

    pub fn create() -> Config<None, None> {
        static NONE: None = None;
        Config {
            model: NONE,
            source: NONE,
        }
    }

    impl<M: Model, S: Source> Config<M,S> {
        pub fn with_model<M1: Model>(self, model: M1) -> Config<M1,S> {
            Config { model, source: self.source }
        }

        pub fn with_source<S1: Source>(self, source: S1) -> Config<M,S1> {
            Config {model: self.model, source}
        }
    }
}

#[derive(Debug)]
struct MyModel;
impl model::Model for MyModel {
    fn mutate(&mut self) {
        println!("mutated");
    }
}

#[derive(Debug)]
struct MySource;
impl source::Source for MySource {
    fn get(&self) {
        println!("get");
    }
}

fn main() {
    use model::Model;
    use source::Source;
    let model = MyModel;
    let source = MySource;
    let c = config::create();
    let c = c.with_model(model);
    let mut c = c.with_source(source);
    c.model.mutate();
    c.source.get();
    println!("{:?}", c);
}
