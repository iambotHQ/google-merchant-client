pub type Identifier=String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shipping{
	#[serde(rename="country")]
	pub country:Option<String>,
	#[serde(rename="price")]
	pub price:Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item{
	#[serde(rename="id")]
	pub id:Identifier,
	#[serde(default)]
	pub title: String,
	#[serde(default)]
	pub description: String,
	#[serde(rename="google_product_category")]
	pub google_product_id:String,
	#[serde(default)]
	pub product_type:String,
	#[serde(default)]
	pub link:String,
	#[serde(rename="image_link",default)]
	pub image:String,
	#[serde(rename="additional_image_link",default)]
	pub additional_images:Vec<String>,
	pub condition:Option<String>,
	#[serde(rename="availability")]
	pub availability:Option<String>,
	#[serde(rename="price",default)]
	pub price:String,
	#[serde(rename="sale_price")]
	pub sale_price:Option<String>,
	#[serde(rename="brand",default)]
	pub brand:String,
	#[serde(rename="color")]
	pub color:Option<String>,
	#[serde(rename="material")]
	pub material:Option<String>,
	#[serde(rename="size")]
	pub size:Option<String>,
	#[serde(rename="shipping")]
	pub shipping:Option<Shipping>,
	#[serde(rename="adult")]
	pub adult:Option<Shipping>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel{
	#[serde(rename="item")]
	pub items:Vec<Item>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename="rss")]
pub struct Feed{
	#[serde(rename = "channel")]
	pub channels:Vec<Channel>
}



