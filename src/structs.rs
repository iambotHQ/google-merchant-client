pub type Identifier=String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Shipping{
	#[serde(rename="g:country")]
	pub country:Option<String>,
	#[serde(rename="g:price")]
	pub price:Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Item{
	#[serde(rename="g:id")]
	pub id:Identifier,
	pub title: String,
	pub description: String,
	#[serde(rename="g:google_product_category")]
	pub google_product_id:String,
	#[serde(rename="g:product_type")]
	pub product_type:String,
	pub link:String,
	#[serde(rename="g:image_link")]
	pub image:String,
	#[serde(rename="g:additional_image_link")]
	pub additional_images:Vec<String>,
	pub condition:Option<String>,
	#[serde(rename="g:availability")]
	pub availability:Option<String>,
	#[serde(rename="g:price")]
	pub price:String,
	#[serde(rename="g:sale_price")]
	pub sale_price:Option<String>,
	#[serde(rename="g:brand")]
	pub brand:String,
	#[serde(rename="g:color")]
	pub color:Option<String>,
	#[serde(rename="g:material")]
	pub material:Option<String>,
	#[serde(rename="g:size")]
	pub size:Option<String>,
	#[serde(rename="g:shipping")]
	pub shipping:Option<Shipping>,
	#[serde(rename="g:adult")]
	pub adult:Option<Shipping>,
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Channel{
	#[serde(rename="item")]
	pub items:Vec<Item>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Feed{
	#[serde(rename = "channel")]
	pub channels:Vec<Channel>
}



