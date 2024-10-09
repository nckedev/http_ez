use std::any::{Any, TypeId};
use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::HashSet;
use std::rc::{Rc, Weak};
use std::sync::Arc;

use crate::server::ServiceScope;

struct Service {
    name: TypeId,
    service: Box<dyn std::any::Any>,
    scope: ServiceScope,
}

#[derive(Hash, PartialEq, Eq)]
struct ServiceId {
    type_id: TypeId,
    request_id: u32,
}

// Define the Factory struct
// struct Factory {
//     objects: HashMap<usize, Weak<RefCell<MyObject>>>, // Weak references to manage objects
// }
//
// impl Factory {
//     fn new() -> Self {
//         Factory {
//             objects: HashMap::new(),
//         }
//     }
//
//     // Factory method to get or create an object
//     fn get_object(&mut self, id: usize) -> Rc<RefCell<MyObject>> {
//         // Check if the object already exists
//         if let Some(weak_ref) = self.objects.get(&id) {
//             if let Some(strong_ref) = weak_ref.upgrade() {
//                 println!("Returning existing object {}.", id);
//                 return strong_ref; // Return existing object
//             }
//         }
//
//         // Create a new object
//         let new_object = Rc::new(RefCell::new(MyObject::new(id)));
//         self.objects.insert(id, Rc::downgrade(&new_object)); // Store weak reference
//         new_object // Return new object
//     }
// }
//
// fn main() {
//     let mut factory = Factory::new();
//
//     let obj1 = factory.get_object(1);
//     let obj2 = factory.get_object(2);
//     let obj3 = factory.get_object(1); // Should return existing object
//
//     // To demonstrate reference count
//     println!("Reference count of obj1: {}", Rc::strong_count(&obj1)); // Returns 2
//     println!("Reference count of obj3: {}", Rc::strong_count(&obj3)); // Returns 2
// }
struct ServiceProvider {
    service_collection: HashMap<ServiceId, Option<Weak<RefCell<dyn Any>>>>,
}

impl ServiceProvider {
    pub fn get<T>(&self) -> Rc<RefCell<T>>
    where
        T: 'static + Any + Default + Clone,
    {
        let type_id = TypeId::of::<T>();
        let id = ServiceId {
            type_id,
            request_id: 0,
        };

        if let Some(key) = self.service_collection.get(&id) {
            if let Some(weak_service) = key {
                if let Some(strong_service) = weak_service.upgrade() {
                    let b = downcast_refcell(strong_service);
                    return b;
                }
            }
        }

        let new_service = Rc::new(RefCell::new(T::default()));
        return new_service;
    }
}

impl From<ServiceCollection> for ServiceProvider {
    fn from(value: ServiceCollection) -> Self {
        let mut provider = ServiceProvider {
            service_collection: HashMap::new(),
        };
        for v in value.service_collection {
            provider.service_collection.insert(
                ServiceId {
                    type_id: v,
                    request_id: 0,
                },
                None,
            );
        }
        provider
    }
}

fn downcast_refcell<T: Any + Default + 'static + Clone>(
    rc: Rc<RefCell<dyn Any>>,
) -> Rc<RefCell<T>> {
    // Attempt to borrow the RefCell and downcast
    if let Ok(refcell) = rc.try_borrow() {
        if let Some(downcasted) = refcell.downcast_ref::<T>() {
            return Rc::new(RefCell::new((*downcasted).clone()));
        }
    }
    panic!("Failed to downcast refcell!")
}
struct ServiceFactory;
impl ServiceFactory {
    pub fn create<T: 'static + Any + Default>() -> T {
        T::default()
    }
}

#[derive(Clone)]
pub struct ServiceCollection {
    service_collection: HashSet<TypeId>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum ServiceCollectionError {
    ServiceAlreadyExists,
}

impl ServiceCollection {
    ///Creates a new ServiceCollection
    pub fn new() -> Self {
        ServiceCollection {
            service_collection: HashSet::new(),
        }
    }

    pub fn generate_provider(&self) -> () {}

    pub fn has_any(&self) -> bool {
        !self.service_collection.is_empty()
    }

    // pub fn get<T: 'static + Any + Default>(&self) -> Option<&T> {
    //     let name = TypeId::of::<T>();
    //     println!("get service : {name:?}");
    //     if let Some(service) = self.service_collection.get(&name) {
    //         // TODO: match scope, requires requestid? for scoped to compare if we need to make a
    //         // new isntance
    //
    //         // match service.scope {
    //         //     ServiceScope::Singleton => todo!(),
    //         //     ServiceScope::Scoped => todo!(),
    //         //     ServiceScope::Transient => todo!(),
    //         // }
    //         // if service.scope == ServiceScope::Transient {
    //         //     let new_service = Service {
    //         //         name,
    //         //         service: Box::new(T::default()),
    //         //         scope: ServiceScope::Transient,
    //         //     };
    //         //
    //         //     *service = new_service;
    //         // }
    //         return service.service.downcast_ref::<T>();
    //     }
    //     None
    // }

    pub fn add<T: 'static + Any + Default>(
        &mut self,
        scope: ServiceScope,
    ) -> Result<(), ServiceCollectionError> {
        let name = TypeId::of::<T>();
        if self.service_collection.contains(&name) {
            return Err(ServiceCollectionError::ServiceAlreadyExists);
        } else {
            self.service_collection.insert(name);
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_service_to_service_collection() {
        #[derive(Default, Clone)]
        struct MyService;

        let mut collection = ServiceCollection::new();
        let add = collection.add::<MyService>(ServiceScope::Scoped);
        assert_eq!(add, Ok(()));
    }

    #[test]
    fn get_service_service_collection() {
        #[derive(Default, Clone)]
        struct MyService;

        impl MyService {
            pub fn return_number(&self) -> u32 {
                1
            }
        }

        let mut collection = ServiceCollection::new();
        let add = collection.add::<MyService>(ServiceScope::Scoped);
        assert_eq!(add, Ok(()));

        let provider = ServiceProvider::from(collection);
        let res = provider.get::<MyService>();
        let r = res.borrow().return_number();

        assert!(matches!(r, 1));
    }

    // #[test]
    // fn get_transient_service_service_collection() {
    //     #[derive(Default)]
    //     struct MyService;
    //
    //     let mut collection = ServiceCollection::new();
    //     let add = collection.add(ServiceScope::Transient);
    //     assert_eq!(add, Ok(()));
    //     let res = collection.get::<MyService>();
    //
    //     assert!(matches!(res, Some(_)));
    // }
}
