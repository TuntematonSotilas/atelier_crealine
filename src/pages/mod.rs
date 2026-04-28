pub mod home;
pub mod not_found;
pub mod services;
pub mod qui_suis_je;
pub mod newsletter;
pub mod ateliers;

pub use home::HomePage;
pub use not_found::NotFound;
pub use services::ServicesPage;
pub use qui_suis_je::QuiSuisJePage;
pub use newsletter::NewsletterPage;
pub use ateliers::{
    AteliersHorsLesAu, AteliersEnInstitution, AteliersParentsEnfants,
    AteliersCreaifsPourTous, AperosCreatifs,
};