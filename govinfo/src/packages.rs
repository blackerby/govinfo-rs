use crate::GovInfo;

pub trait Packages {
    fn md5(self, md5: String) -> Self;
    fn granule_class(self, granule_class: String) -> Self;
    fn package_id(self, package_id: String) -> Self;
    fn summary(self) -> Self;
    fn granules(self) -> Self;
    fn granule_id(self, granule_id: String) -> Self;
}

impl Packages for GovInfo {
    fn md5(mut self, md5: String) -> Self {
        self.params.insert("md5".to_string(), md5.to_string());
        self
    }

    fn granule_class(mut self, granule_class: String) -> Self {
        self.params
            .insert("granuleClass".to_string(), granule_class.to_string());
        self
    }

    fn package_id(mut self, package_id: String) -> Self {
        self.path_params.push(package_id);
        self
    }

    fn summary(mut self) -> Self {
        self.path_params.push(String::from("summary"));
        self
    }

    fn granules(mut self) -> Self {
        self.path_params.push(String::from("granules"));
        self
    }

    fn granule_id(mut self, granule_id: String) -> Self {
        self.path_params.push(granule_id);
        self
    }
}
