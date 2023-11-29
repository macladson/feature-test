use macro_test::generate_feature_with_dependencies;
use std::fmt::Debug;

///
/// We ideally should have both insert order preservation AND deduplication.
/// We should therefore use HashSet (or ideally a LinkedHashSet) here. However, my attempts led
/// to non-trival trait bound issues, so I am leaving this as a Vec for now.
///
pub type Features = Vec<Box<dyn Feature>>;

pub trait Feature: Debug {
    fn dependencies(&self) -> Features;

    fn dependencies_flat(&self) -> Features {
        let mut result = vec![];
        for dependency in self.dependencies() {
            // Add nested dependencies first.
            result.append(&mut dependency.dependencies_flat());

            // Dedup here?

            // Then add self.
            result.push(dependency);
        }
        result
    }
}

pub fn feature_order(order: Features) -> Features {
    let mut result = vec![];
    for feat in order {
        result.append(&mut feat.dependencies_flat());
        result.push(feat);
    }
    result
}

//
// Features
//
#[derive(Debug)]
#[generate_feature_with_dependencies([])]
pub struct EIP100;

#[derive(Debug)]
#[generate_feature_with_dependencies([])]
struct EIP150;

#[derive(Debug)]
#[generate_feature_with_dependencies([EIP150])]
struct EIP200;

#[derive(Debug)]
#[generate_feature_with_dependencies([])]
struct EIP300;

#[derive(Debug)]
#[generate_feature_with_dependencies([EIP200])]
struct EIP400;

//Forks
#[derive(Debug)]
#[generate_feature_with_dependencies([EIP100, EIP200])]
struct Altair;

#[derive(Debug)]
#[generate_feature_with_dependencies([Altair, EIP300, EIP400])]
struct Bellatrix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn show_feature_order() {

        let order = feature_order(vec![Box::new(Altair), Box::new(Bellatrix)]);

        eprintln!("{:?}", order);
    }
}
