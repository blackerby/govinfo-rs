use core::fmt;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub struct CollectionResponse {
    collection_code: Collection,
    collection_name: String,
    package_count: usize,
    granule_count: Option<usize>,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize)]
pub enum Collection {
    Bills,
    BillStatus,
    Budget,
    Ccal,
    Cdir,
    Cdoc,
    Cfr,
    Chrg,
    Cmr,
    Comps,
    Cpd,
    Cprt,
    Crec,
    Crecb,
    Cri,
    Crpt,
    Czic,
    Ecfr,
    Econi,
    Eric,
    Erp,
    Fr,
    GaoReports,
    GovMan,
    GovPub,
    Gpo,
    HJournal,
    HMan,
    Hob,
    Lsa,
    Pai,
    Plaw,
    Ppp,
    SerialSet,
    SJournal,
    SMan,
    Statute,
    UsCode,
    UsCourts,
}

// there has got to be a better way to do this
impl fmt::Display for Collection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data = match *self {
            Self::Bills => "BILLS",
            Self::BillStatus => "BILLSTATUS",
            Self::Budget => "BUDGET",
            Self::Ccal => "CCAL",
            Self::Cdir => "CDIR",
            Self::Cdoc => "CDOC",
            Self::Cfr => "CFR",
            Self::Chrg => "CHRG",
            Self::Cmr => "CMR",
            Self::Comps => "COMPS",
            Self::Cpd => "CPD",
            Self::Cprt => "CPRT",
            Self::Crec => "CREC",
            Self::Crecb => "CRECB",
            Self::Cri => "CRI",
            Self::Crpt => "CRPT",
            Self::Czic => "CZIC",
            Self::Ecfr => "ECFR",
            Self::Econi => "ECONI",
            Self::Eric => "ERIC",
            Self::Erp => "ERP",
            Self::Fr => "FR",
            Self::GaoReports => "GAOREPORTS",
            Self::GovMan => "GOVMAN",
            Self::GovPub => "GOVPUB",
            Self::Gpo => "GPO",
            Self::HJournal => "HJOURNAL",
            Self::HMan => "HMAN",
            Self::Hob => "HOB",
            Self::Lsa => "LSA",
            Self::Pai => "PAI",
            Self::Plaw => "PLAW",
            Self::Ppp => "PPP",
            Self::SerialSet => "SERIALSET",
            Self::SJournal => "SJOURNAL",
            Self::SMan => "SMAN",
            Self::Statute => "STATUTE",
            Self::UsCode => "USCODE",
            Self::UsCourts => "USCOURTS",
        };

        f.write_str(data)
    }
}

// there has got to be a better way to do this
impl Collection {
    pub fn name(&self) -> String {
        let data = match *self {
            Self::Bills => "Congressional Bills",
            Self::BillStatus => "Congressional Bill Status",
            Self::Budget => "United States Budget",
            Self::Ccal => "Congressional Calendars",
            Self::Cdir => "Congressional Directory",
            Self::Cdoc => "Congressional Directory",
            Self::Cfr => "Code of Federal Regulations",
            Self::Chrg => "Congressional Hearings",
            Self::Cmr => "Congressionally Mandated Reports",
            Self::Comps => "Statutes Compilations",
            Self::Cpd => "Compilation of Presidential Documents",
            Self::Cprt => "Congressional Committee Prints",
            Self::Crec => "Congressional Record",
            Self::Crecb => "Congressional Record (Bound Editions)",
            Self::Cri => "Congressional Record Index",
            Self::Crpt => "Congressional Reports",
            Self::Czic => "Coastal Zone Information Center",
            Self::Ecfr => "Electronic Code of Federal Regulations",
            Self::Econi => "Economic Indicators",
            Self::Eric => "Education Reports from ERIC",
            Self::Erp => "Economic Report of the President",
            Self::Fr => "Federal Register",
            Self::GaoReports => {
                "Government Accountability Office Reports and Comptroller General Decisions"
            }
            Self::GovMan => "United States Government Manual",
            Self::GovPub => "Bulk Submission",
            Self::Gpo => "Additional Government Publications",
            Self::HJournal => "Journal of the House of Representatives",
            Self::HMan => "House Rules and Manual",
            Self::Hob => "History of Bills",
            Self::Lsa => "List of CFR Sections Affected",
            Self::Pai => "Privacy Act Issuances",
            Self::Plaw => "Public and Private Laws",
            Self::Ppp => "Public Papers of Presidents of the United States",
            Self::SerialSet => "Congressional Serial Set",
            Self::SJournal => "Journal of the Senate",
            Self::SMan => "Senate Manual",
            Self::Statute => "Statutes at Large",
            Self::UsCode => "United States Code",
            Self::UsCourts => "United States Courts Opinions",
        };

        data.to_string()
    }
}
