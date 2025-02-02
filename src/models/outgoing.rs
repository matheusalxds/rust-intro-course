use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize)]
pub struct OutgoingIdentifier {
	pub concatenated_string: String,
	pub color: String,
	pub product_name: String,
	pub warehouse: String,
	pub location: String,
	pub pcs: i32,
}

#[derive(Validate, Deserialize, Serialize)]
pub struct RemoveUniqueIdentifierRequest {
	#[validate(length(min = 1, message = "Color is required"))]
	pub color: String,
	#[validate(length(min = 1, message = "Product name is required"))]
	pub product_name: String,
	#[validate(length(min = 1, message = "Warehouse is required"))]
	pub warehouse: String,
	#[validate(length(min = 1, message = "Location is required"))]
	pub location: String,
	#[validate(range(min = 1, max = 10000, message = "PCS must be between 1 and 10000"))]
	pub pcs: i32,
}