// Copyright 2025 Cowboy AI, LLC.

//! Private Mortgage Lending - Document Collection and Validation
//!
//! This example demonstrates document routing, OCR processing, validation
//! workflows, and expiration tracking for mortgage lending documents.

use std::collections::HashMap;
use std::sync::Arc;

use chrono::{
    DateTime,
    Duration,
    Utc,
};
use cim_subject::{
    translator::{
        TranslationRule,
        Translator,
    },
    AlgebraOperation,
    Pattern,
    Subject,
    SubjectAlgebra,
    SubjectBuilder,
};

#[derive(Debug, Clone)]
struct Document {
    id: String,
    doc_type: DocumentType,
    received_date: DateTime<Utc>,
    broker_id: String,
    loan_id: String,
    status: DocumentStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum DocumentType {
    Paystub,
    W2,
    TaxReturn,
    BankStatement,
    Appraisal,
    TitleReport,
    Insurance,
    DriverLicense,
    SocialSecurityCard,
}

#[derive(Debug, Clone, PartialEq)]
enum DocumentStatus {
    Received,
    OcrProcessing,
    Validating,
    Approved,
    Rejected,
    Expired,
}

impl DocumentType {
    fn expiration_days(&self) -> i64 {
        match self {
            DocumentType::Paystub => 30,
            DocumentType::W2 => 365,
            DocumentType::TaxReturn => 365,
            DocumentType::BankStatement => 60,
            DocumentType::Appraisal => 120,
            DocumentType::TitleReport => 90,
            DocumentType::Insurance => 365,
            DocumentType::DriverLicense => 1825,      // 5 years
            DocumentType::SocialSecurityCard => 3650, // 10 years
        }
    }

    fn validation_level(&self) -> &str {
        match self {
            DocumentType::TaxReturn | DocumentType::Appraisal => "enhanced",
            DocumentType::BankStatement | DocumentType::TitleReport => "standard",
            _ => "basic",
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Document Validation Workflow Example ===\n");

    // Example 1: Document intake routing
    println!("1. Document Intake Routing:\n");

    let documents = vec![
        Document {
            id: "DOC-001".to_string(),
            doc_type: DocumentType::Paystub,
            received_date: Utc::now() - Duration::days(5),
            broker_id: "BRK-GOLD-001".to_string(),
            loan_id: "LOAN-12345".to_string(),
            status: DocumentStatus::Received,
        },
        Document {
            id: "DOC-002".to_string(),
            doc_type: DocumentType::TaxReturn,
            received_date: Utc::now() - Duration::days(10),
            broker_id: "BRK-PLAT-001".to_string(),
            loan_id: "LOAN-12345".to_string(),
            status: DocumentStatus::OcrProcessing,
        },
        Document {
            id: "DOC-003".to_string(),
            doc_type: DocumentType::Appraisal,
            received_date: Utc::now() - Duration::days(100),
            broker_id: "BRK-GOLD-001".to_string(),
            loan_id: "LOAN-12346".to_string(),
            status: DocumentStatus::Approved,
        },
        Document {
            id: "DOC-004".to_string(),
            doc_type: DocumentType::DriverLicense,
            received_date: Utc::now() - Duration::days(200),
            broker_id: "BRK-GOLD-001".to_string(),
            loan_id: "LOAN-12345".to_string(),
            status: DocumentStatus::Validating,
        },
        Document {
            id: "DOC-005".to_string(),
            doc_type: DocumentType::SocialSecurityCard,
            received_date: Utc::now() - Duration::days(50),
            broker_id: "BRK-GOLD-001".to_string(),
            loan_id: "LOAN-12345".to_string(),
            status: DocumentStatus::Rejected,
        },
        Document {
            id: "DOC-006".to_string(),
            doc_type: DocumentType::BankStatement,
            received_date: Utc::now() - Duration::days(70),
            broker_id: "BRK-PLAT-001".to_string(),
            loan_id: "LOAN-12346".to_string(),
            status: DocumentStatus::Expired,
        },
    ];

    for doc in &documents {
        route_document(doc)?;
    }

    // Example 2: Expiration checking
    println!("\n2. Document Expiration Status:\n");

    for doc in &documents {
        check_expiration(doc)?;
    }

    // Example 3: Document collection tracking
    println!("\n3. Document Collection Status:\n");

    let required_docs = vec![
        DocumentType::Paystub,
        DocumentType::W2,
        DocumentType::TaxReturn,
        DocumentType::BankStatement,
        DocumentType::Appraisal,
    ];

    let collected: Vec<_> = documents
        .iter()
        .filter(|d| d.loan_id == "LOAN-12345")
        .map(|d| &d.doc_type)
        .collect();

    for doc_type in &required_docs {
        let status = if collected.contains(&doc_type) {
            "✓"
        } else {
            "✗"
        };
        println!("  {status} {doc_type:?}");
    }

    // Example 4: OCR workflow translation
    println!("\n\n4. OCR Processing Workflow:\n");

    let ocr_translator = create_ocr_translator()?;

    let ocr_subjects = vec![
        "lending.documents.raw.paystub.uploaded",
        "lending.documents.raw.w2.uploaded",
        "lending.documents.raw.bank_statement.uploaded",
    ];

    for subject_str in ocr_subjects {
        let subject = Subject::new(subject_str)?;
        let ocr_subject = ocr_translator.translate(&subject)?;
        println!("  {} → {}", subject.as_str(), ocr_subject.as_str());
    }

    // Example 5: Validation rules by document type
    println!("\n\n5. Document Validation Rules:\n");

    let validation_rules = create_validation_rules();

    for (doc_type, rules) in &validation_rules {
        println!("\n  {doc_type:?}:");
        for (rule_name, subject_pattern) in rules {
            println!("    - {rule_name}: {subject_pattern}");
        }
    }

    // Example 6: Multi-step validation workflow
    println!("\n\n6. Multi-Step Validation Workflow:\n");

    let algebra = SubjectAlgebra::new();

    // Register validation transformations
    algebra.register_transformation("basic_validation", cim_subject::algebra::Transformation {
        name: "basic_validation".to_string(),
        input_pattern: Pattern::new("lending.documents.*.*.received")?,
        transform: Arc::new(|subject| {
            let parts = cim_subject::SubjectParts::parse(subject.as_str())?;
            let validated = cim_subject::SubjectParts::new(
                parts.context,
                "validation",
                format!("{}_basic", parts.aggregate),
                "v1",
            );
            Ok(Subject::from_parts(validated))
        }),
    });

    let test_doc = Subject::new("lending.documents.income.paystub.received")?;
    let validated = algebra.compose(&test_doc, &test_doc, AlgebraOperation::Transform {
        name: "basic_validation".to_string(),
    })?;

    println!("  {} → {}", test_doc.as_str(), validated.as_str());

    // Example 7: Document routing patterns
    println!("\n\n7. Document Routing Patterns:\n");

    let routing_patterns = vec![
        (
            "Income Documents",
            Pattern::new("lending.documents.income.>")?,
        ),
        (
            "Property Documents",
            Pattern::new("lending.documents.property.>")?,
        ),
        (
            "Identity Documents",
            Pattern::new("lending.documents.identity.>")?,
        ),
        (
            "All Validated",
            Pattern::new("lending.validation.*.approved")?,
        ),
        (
            "All Rejected",
            Pattern::new("lending.validation.*.rejected")?,
        ),
    ];

    let test_subjects = vec![
        "lending.documents.income.paystub.received",
        "lending.documents.property.appraisal.received",
        "lending.documents.identity.license.received",
        "lending.validation.income.approved",
        "lending.validation.property.rejected",
    ];

    for (pattern_name, pattern) in &routing_patterns {
        println!("\n  Pattern: {} ({})", pattern_name, pattern.as_str());
        for subject_str in &test_subjects {
            let subject = Subject::new(*subject_str)?;
            if pattern.matches(&subject) {
                println!("    ✓ {subject_str}");
            }
        }
    }

    // Example 8: Document compliance by state
    println!("\n\n8. State-Specific Document Requirements:\n");

    // First show missing documents for a loan
    let missing = identify_missing_documents("LOAN-12345");
    if !missing.is_empty() {
        println!("  Missing documents for loan LOAN-12345:");
        for doc_type in &missing {
            println!("    - {doc_type:?}");
        }
    }

    let state_requirements = HashMap::from([
        ("CA", vec![
            DocumentType::TitleReport,
            DocumentType::Insurance,
        ]),
        ("TX", vec![DocumentType::TitleReport]),
        ("NY", vec![
            DocumentType::TitleReport,
            DocumentType::Insurance,
            DocumentType::Appraisal,
        ]),
    ]);

    for (state, required) in state_requirements {
        println!("\n  State: {state}");
        for doc_type in required {
            let subject = SubjectBuilder::new()
                .context("lending")
                .aggregate("compliance")
                .event_type(
                    format!("{}_required_{:?}", state.to_lowercase(), doc_type).to_lowercase(),
                )
                .version("v1")
                .build()?;
            println!("    {:?} → {}", doc_type, subject.as_str());
        }
    }

    Ok(())
}

fn route_document(doc: &Document) -> Result<(), Box<dyn std::error::Error>> {
    let doc_category = match doc.doc_type {
        DocumentType::Paystub | DocumentType::W2 | DocumentType::TaxReturn => "income",
        DocumentType::BankStatement => "assets",
        DocumentType::Appraisal | DocumentType::TitleReport => "property",
        DocumentType::Insurance => "insurance",
        DocumentType::DriverLicense | DocumentType::SocialSecurityCard => "identity",
    };

    let subject = SubjectBuilder::new()
        .context(format!("lending.{}.{}", doc.broker_id, doc_category))
        .aggregate("documents")
        .event_type(format!("{:?}", doc.doc_type).to_lowercase())
        .version("v1")
        .build()?;

    println!(
        "  Document {} ({:?}) → {}",
        doc.id,
        doc.doc_type,
        subject.as_str()
    );
    println!(
        "    Status: {:?}, Validation Level: {}",
        doc.status,
        doc.doc_type.validation_level()
    );

    Ok(())
}

fn check_expiration(doc: &Document) -> Result<(), Box<dyn std::error::Error>> {
    let age_days = (Utc::now() - doc.received_date).num_days();
    let expiry_days = doc.doc_type.expiration_days();
    let days_remaining = expiry_days - age_days;

    let status = if days_remaining < 0 {
        "EXPIRED"
    } else if days_remaining < 30 {
        "EXPIRING SOON"
    } else {
        "VALID"
    };

    println!(
        "  {} ({:?}): {} - {} days old, {} days remaining",
        doc.id,
        doc.doc_type,
        status,
        age_days,
        days_remaining.max(0)
    );

    // Create expiration event if needed
    if days_remaining < 0 {
        let expiry_subject = SubjectBuilder::new()
            .context("lending")
            .aggregate("documents")
            .event_type("expired")
            .version("v1")
            .build()?;
        println!("    → Event: {}", expiry_subject.as_str());
    }

    Ok(())
}

fn create_ocr_translator() -> Result<Translator, Box<dyn std::error::Error>> {
    let translator = Translator::new();

    // Translate raw uploads to OCR queue
    translator.register_rule(
        "raw_to_ocr",
        TranslationRule::new(
            "raw_to_ocr",
            Pattern::new("lending.documents.raw.*.uploaded")?,
            Arc::new(|subject| {
                let new_str = subject
                    .as_str()
                    .replace(".raw.", ".ocr.")
                    .replace(".uploaded", ".queued");
                Subject::new(new_str)
            }),
        ),
    );

    // Translate OCR complete to validation
    translator.register_rule(
        "ocr_to_validation",
        TranslationRule::new(
            "ocr_to_validation",
            Pattern::new("lending.documents.ocr.*.completed")?,
            Arc::new(|subject| {
                let new_str = subject
                    .as_str()
                    .replace(".ocr.", ".validation.")
                    .replace(".completed", ".pending");
                Subject::new(new_str)
            }),
        ),
    );

    // Translate validation results
    translator.register_rule(
        "validation_result",
        TranslationRule::new(
            "validation_result",
            Pattern::new("lending.documents.validation.*.checked")?,
            Arc::new(|subject| {
                let new_str = subject
                    .as_str()
                    .replace(".validation.", ".status.")
                    .replace(".checked", ".final");
                Subject::new(new_str)
            }),
        ),
    );

    Ok(translator)
}

fn create_validation_rules() -> HashMap<DocumentType, Vec<(&'static str, &'static str)>> {
    let mut rules = HashMap::new();

    rules.insert(DocumentType::Paystub, vec![
        ("Date Check", "lending.validation.date.within_30_days"),
        ("Employer Match", "lending.validation.employer.verified"),
        ("Amount Check", "lending.validation.income.reasonable"),
    ]);

    rules.insert(DocumentType::BankStatement, vec![
        (
            "Account Verification",
            "lending.validation.account.ownership",
        ),
        ("Balance Check", "lending.validation.balance.sufficient"),
        (
            "Transaction Analysis",
            "lending.validation.transactions.analyzed",
        ),
        ("NSF Check", "lending.validation.nsf.none_found"),
    ]);

    rules.insert(DocumentType::Appraisal, vec![
        ("Appraiser License", "lending.validation.appraiser.licensed"),
        (
            "Value Reasonableness",
            "lending.validation.value.reasonable",
        ),
        ("Comparables Check", "lending.validation.comps.verified"),
        (
            "Property Match",
            "lending.validation.property.matches_application",
        ),
    ]);

    rules
}

fn identify_missing_documents(loan_id: &str) -> Vec<DocumentType> {
    // In a real implementation, this would query the database
    println!("    Checking database for loan: {loan_id}");
    vec![DocumentType::W2, DocumentType::TitleReport]
}
