mod repository;
mod types;

// Re-export types
pub use types::{NewProperty, Property, PropertyValue, TypedProperty};

// Re-export repository functions
pub use repository::{get_properties, get_property, get_property_count, get_property_value_or};
