use soroban_sdk::{contracterror, contracttype, Address, String, Symbol, Vec};

#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum AdminError {
    AlreadyVerified = 1,
    UnauthorizedAccess = 2,
}

#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum AuctionError {
    BidTooLow = 1,
    AuctionEnded = 2,
    AuctionAlreadyExists = 3,
    InvalidBidder = 4,
    AuctionNotFound = 5,
    TooLateToExtend = 6,
    InvalidAuctionEndTime = 7,
    AuctionNotYetEnded = 8,
    NoBidsPlaced = 9,
    ProductNotFound = 10,
    OutOfStock = 11,
}

#[contracttype]
#[derive(Clone)]
pub struct Auction {
    pub product_id: u64,
    pub highest_bid: u64,
    pub highest_bidder: Option<Address>,
    pub reserve_price: u64,
    pub auction_end_time: u64,
    pub seller: Address,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKeys {
    Admin,
    Auction(Address, u64),          // Sellers Created Auctions
    ProductList(Address),           // ProductList of Seller
    Product(Address, u64),          // Product related to Seller
    ShipmentList(Address),          // ShipmentList of Seller
    Shipment(Address, String),      // Shipment related to Seller
    SellerVerification(Address),    // Seller Verification
    Dispute(Address, Address, u64), // Dispute related to Buyer and Seller and Product_id
    ReturnPolicy(Address),          // Return Policy of Seller,
    ReturnRequest(Address, u64),    // Return Request related to Seller
}

#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum ProductError {
    InvalidDescription = 1,
    InvalidPrice = 2,
    InvalidWeight = 3,
    OutOfStock = 4,
    InvalidImageCount = 5,
    ProductNotFound = 6,
    Unauthorized = 7,
    ReturnPolicyNotFound = 8,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum Condition {
    New,
    OpenBox,
    UsedGood,
    UsedAcceptable,
    Refurbished,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Product {
    pub id: u64,
    pub seller: Address,
    pub name: Symbol,
    pub description: String,
    pub price: u64,
    pub condition: Condition,
    pub stock: u32,
    pub images: Vec<String>,
    pub weight_pounds: u64,
    pub verified: bool,
}

#[contracttype]
#[derive(Clone, Debug)]
pub struct Shipment {
    pub seller: Address,
    pub buyer: Address,
    pub weight_pounds: u32,
    pub distance_km: u32,
    pub shipping_cost: u64,
    pub delivery_estimate_days: u32,
    pub status: Symbol,
    pub tracking_number: String,
}

pub const COST_PER_POUND: u64 = 6;
pub const COST_PER_KM: u64 = 1;

#[contracterror]
#[derive(Debug, Clone, PartialEq)]
pub enum ShippingError {
    RestrictedLocation = 1,
    ShipmentNotFound = 2,
    ShipmentAlreadyExists = 3,
    InvalidBuyerZone = 4,
}

#[contracttype]
#[derive(Clone)]
pub struct Dispute {
    pub buyer: Address,
    pub seller: Address,
    pub product_id: u64,
    pub reason: String,
    pub status: DisputeStatus,
}

#[contracttype]
#[derive(Clone)]
pub struct ReturnRequest {
    pub buyer: Address,
    pub seller: Address,
    pub product_id: u64,
    pub reason: String,
    pub status: Symbol,
}

#[contracterror]
#[derive(Clone)]
pub enum VerificationError {
    ProductNotFound = 1,
    AlreadyRequested = 2,
    NoVerificationRequest = 3,
    DisputeAlreadyExists = 4,
    DisputeNotFound = 5,
    ReturnAlreadyRequested = 6,
    RestrictedLocation = 7,
    ReturnRequestNotFound = 8,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum SellerVerificationStatus {
    Rejected,
    Verified,
    Pending,
}

#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub enum DisputeStatus {
    Rejected,
    Approved,
    Pending,
}
