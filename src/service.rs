use std::any::{Any, TypeId};
use std::collections::HashMap;

use crate::server::ServiceScope;

struct Service {
    name: TypeId,
    service: Box<dyn std::any::Any>,
    scope: ServiceScope,
}

struct ServiceProvider {}

pub struct ServiceCollection {
    service_collection: HashMap<TypeId, Service>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ServiceCollectionError {
    ServiceAlreadyExists,
}

impl ServiceCollection {
    ///Creates a new ServiceCollection
    pub fn new() -> Self {
        ServiceCollection {
            service_collection: HashMap::new(),
        }
    }

    pub fn generate_provider(&self) -> () {}

    pub fn has_any(&self) -> bool {
        !self.service_collection.is_empty()
    }

    pub fn get<T: 'static + Any + Default>(&self) -> Option<&T> {
        let name = TypeId::of::<T>();
        println!("get service : {name:?}");
        if let Some(service) = self.service_collection.get(&name) {
            // TODO: match scope, requires requestid? for scoped to compare if we need to make a
            // new isntance

            // match service.scope {
            //     ServiceScope::Singleton => todo!(),
            //     ServiceScope::Scoped => todo!(),
            //     ServiceScope::Transient => todo!(),
            // }
            // if service.scope == ServiceScope::Transient {
            //     let new_service = Service {
            //         name,
            //         service: Box::new(T::default()),
            //         scope: ServiceScope::Transient,
            //     };
            //
            //     *service = new_service;
            // }
            return service.service.downcast_ref::<T>();
        }
        None
    }

    pub fn add<T: 'static + Any + Default>(
        &mut self,
        service: T,
        scope: ServiceScope,
    ) -> Result<(), ServiceCollectionError> {
        let name = TypeId::of::<T>();
        if self.service_collection.contains_key(&name) {
            return Err(ServiceCollectionError::ServiceAlreadyExists);
        } else {
            let s = Service {
                name,
                service: Box::new(service),
                scope,
            };
            self.service_collection.insert(name, s);
            return Ok(());
        }
    }

    fn create_service<T: 'static + Any + Default>() -> Option<T> {
        Some(T::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_service_to_service_collection() {
        #[derive(Default)]
        struct MyService;

        let mut collection = ServiceCollection::new();
        let add = collection.add(MyService, ServiceScope::Scoped);
        assert_eq!(add, Ok(()));
    }

    #[test]
    fn get_service_service_collection() {
        #[derive(Default)]
        struct MyService;

        let mut collection = ServiceCollection::new();
        let add = collection.add(MyService, ServiceScope::Scoped);
        assert_eq!(add, Ok(()));

        let res = collection.get::<MyService>();

        assert!(matches!(res, Some(_)));
    }

    #[test]
    fn get_transient_service_service_collection() {
        #[derive(Default)]
        struct MyService;

        let mut collection = ServiceCollection::new();
        let add = collection.add(MyService, ServiceScope::Transient);
        assert_eq!(add, Ok(()));
        let res = collection.get::<MyService>();

        assert!(matches!(res, Some(_)));
    }
}
