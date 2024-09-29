use uuid::Uuid;


#[derive(Default, Debug)]
pub struct Depense {
    pub id: Uuid,
    pub libelle: String,
    pub montant: f32
} 

