// --------------------------------------------------
// tinyklv
// --------------------------------------------------
use tinyklv::Klv;
use tinyklv::prelude::*;

#[cfg(any(
    feature = "misb0903-6",
))]
#[derive(Klv, Debug)]
#[klv(
    // ------------------------------------------------
    // There is no UL - Local Set only
    // ------------------------------------------------
    stream = &[u8],
    key(enc = tinyklv::codecs::ber::enc::ber_oid,
        dec = tinyklv::codecs::ber::dec::ber_oid::<u64>),
    len(enc = tinyklv::codecs::ber::enc::ber_length,
        dec = tinyklv::codecs::ber::dec::ber_length),
    default(ty = u64, dyn = true, dec = tinyklv::codecs::binary::dec::be_u64_lengthed),
    default(ty = String, dyn = true, dec = tinyklv::codecs::binary::dec::to_string_utf8),
)]
/// MISB 0903 VMTI Ontology LS (Local Set)
/// 
/// `ontologySeries` -> [`Vec<Misb0903Ontology>`]
/// 
/// The `ontologySeries` item is a Series type which contains one or more Ontology Local Sets. The
/// length for the series is the number of bytes the `ontologySeries` value uses. The value is comprised
/// of one or more Ontology LS, each of which can be of a different size (thereby including different
/// information) parsed according to the length provided for each LS.
/// 
/// The `ontologySeries` enables assigning multiple ontologies to each [`VTarget`](crate::misb0903::target::VTarget)
/// (see [`VTarget`](crate::misb0903::target::VTarget) Item 107) and multiple features to each 
/// [`VObject`](crate::misb0903::target::Misb0903Object) (see [`VObject`](crate::misb0903::target::Misb0903Object)
/// Item 5). 
/// 
/// The Ontology LS describes the class or type of a target (aircraft, watercraft, car, truck, train,
/// dismount, etc.) to an arbitrary level of detail. This standard mandates the use of the Web
/// Ontology Language (OWL) [4] to define the ontology, see Section 7.3.
/// 
/// For more information, see [Motion Imagery Standards Board (MISB)](https://nsgreg.nga.mil/misb.jsp)
pub struct Misb0903Ontology {
    #[klv(key = 0x01)]
    /// (Mandatory) Identifier for the ontology used
    /// 
    /// `ontologyId` -> [`Misb0903Ontology::ontology_id`]
    /// 
    /// When the VMTI LS contains a Series of Ontology LS, each element in the Series gets a unique
    /// identifier. The `ontologyId` item is an integer which identifies a single Ontology LS in the Series
    /// and is unique in the list. Systems may reuse `ontologyId`s for different ontologies/entities from
    /// VMTI LS to VMTI LS (i.e., two sequential VMTI packets) so receivers should not assume an
    /// `ontologyId` value is static for a whole VMTI stream. The `ontologyId` does not need to start at a
    /// value of one nor do the `ontologyId`s need to be in any specific order in the Ontology Series.
    /// 
    /// Len: V8
    pub ontology_id: u64,

    #[klv(key = 0x02)]
    /// (Optional) Defines the link when an `ontologySeries` has two related LS in the Series
    /// 
    /// `parentId` -> [`Misb0903Ontology::parent_id`]
    /// 
    /// The `parentId` enables relating one ontology reference to another. When detecting or tracking
    /// objects, there may be several related object-labels made for an object. From the example in
    /// Section 7.3, vehicle is a generalization of both car and motorcycle. Systems may use the `parentId`
    /// item to indicate the parent-child relationships between car and vehicle, motorcycle, and vehicle.
    /// 
    /// However, this information is duplicative to the ontology hierarchy; therefore starting with
    /// ST0903.6 the MISP suggests not using this item.
    /// When an `ontologySeries` has two related Ontology LS in the Series, the parentId defines the link
    /// by the child defining its parentId equal to the parent ontology object's `ontologyId`. For example,
    /// consider an `ontologySeries` having three elements: vehicle with `ontologyId` 10, car with
    /// `ontologyId` 17, and motorcycle with `ontologyId` 3. Since car and motorcycle are both "children"
    /// of vehicle, those two LS define their parentId as equal to 10.
    /// 
    /// Len: V8
    pub parent_id: Option<u64>,

    #[klv(key = 0x03)]
    /// (Mandatory) Internationalized Resource Identifier (IRI) which identifies the OWL ontology
    /// for the `entityIRI`.
    /// 
    /// `ontologyIRI` -> [`Misb0903Ontology::onology_iri`]
    /// 
    /// The `ontologyIRI` identifies the ontology which provides the definition of the `entityIRI`. See
    /// Section 7.3. 
    pub onology_iri: String,

    #[klv(key = 0x04)]
    /// (Mandatory) Internationalized Resource Identifier (IRI) specifying an entity from the
    /// ontology.
    /// 
    /// `entityIRI` -> [`Misb0903Ontology::entity_iri`]
    /// 
    /// The `entityIRI` identifies an entity within the ontology. See Section 7.3
    pub entity_iri: String,

    #[klv(key = 0x05)]
    /// (Optional) Internationalized Resource Identifier (IRI) specifying the version of the
    /// ontology. 
    /// 
    /// `versionIRI` -> [`Misb0903Ontology::version_iri`]
    /// 
    /// The `versionIRI` identifies the ontology version. See Section 7.3.
    pub version_iri: Option<String>,

    #[klv(key = 0x06)]
    /// (Optional) Entity label defined in the ontology.
    /// 
    /// `label` -> [`Misb0903Ontology::label`]
    /// 
    /// The `label` is the name of the entity, defined in the entityIRI, as defined by the ontology. See
    /// Section 7.3. The `label` text is either the value of the (OWL defined) `rdfs:label`
    /// or `skos:prefLabel`
    /// property of the entity
    pub label: Option<String>,
}