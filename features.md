serve http

service registry

routing
    groups
    versioning

tracing 
    opentelemetry
    
middleware with access to services 

AspNet middlware order 
ExceptionHandler -> HSTS -> HttpsRedirection -> Static Files -> Routing -> Cors -> Authentication -> Authorization -> custom -> Endpoint
