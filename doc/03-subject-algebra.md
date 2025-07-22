<!-- Copyright (c) 2025 Cowboy AI, LLC. -->
# Subject Algebra

## Introduction

The Subject Algebra in CIM provides a mathematical framework for composing and manipulating message subjects. This algebra enables sophisticated routing patterns, workflow orchestration, and system composition while maintaining formal properties that ensure correctness and predictability.

## Algebraic Operations Overview

```mermaid
graph TB
    subgraph "Subject Algebra Operations"
        S[Subject]
        S --> SEQ[Sequential →]
        S --> PAR[Parallel ⊗]
        S --> CHO[Choice ⊕]
        S --> LAT[Lattice Ops]
        
        SEQ --> SEQP[A → B → C]
        PAR --> PARP[A ⊗ B ⊗ C]
        CHO --> CHOP[A ⊕ B ⊕ C]
        LAT --> JOIN[Join ⊔]
        LAT --> MEET[Meet ⊓]
    end
    
    subgraph "Properties"
        SEQP --> P1[Associative<br/>Non-Commutative]
        PARP --> P2[Associative<br/>Commutative]
        CHOP --> P3[Associative<br/>Commutative]
        JOIN --> P4[Least Upper Bound]
        MEET --> P5[Greatest Lower Bound]
    end
    
    style S fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style SEQ fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style PAR fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CHO fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style LAT fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

## Foundational Concepts

### Subject as Algebraic Structure

A subject in CIM forms an algebraic structure with the following properties:

```rust
pub trait SubjectAlgebra {
    type Subject;
    
    // Identity element
    fn identity() -> Self::Subject;
    
    // Binary operations
    fn compose(&self, other: &Self::Subject) -> Self::Subject;
    fn parallel(&self, other: &Self::Subject) -> Self::Subject;
    fn choice(&self, other: &Self::Subject) -> Self::Subject;
    
    // Properties verification
    fn is_associative(&self) -> bool;
    fn is_commutative(&self) -> bool;
    fn has_identity(&self) -> bool;
}
```

## Core Operations

### 1. Sequential Composition (→)

Sequential composition represents ordered execution where one operation must complete before the next begins.

#### Sequential Flow Visualization

```mermaid
graph LR
    subgraph "Sequential Composition: A → B → C"
        A[Subject A<br/>order.commands.create] 
        B[Subject B<br/>payment.commands.process]
        C[Subject C<br/>shipping.commands.dispatch]
        
        A -->|completes| B
        B -->|completes| C
    end
    
    subgraph "Timeline"
        T1[T1: A starts] --> T2[T2: A completes<br/>B starts]
        T2 --> T3[T3: B completes<br/>C starts]
        T3 --> T4[T4: C completes]
    end
    
    style A fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style B fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style C fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
```

#### Associativity Property

```mermaid
graph TB
    subgraph "Left Association: (A → B) → C"
        L1[A → B] --> L2[Result]
        L2 --> L3[Result → C]
        L3 --> LF[Final]
    end
    
    subgraph "Right Association: A → (B → C)"
        R1[B → C] --> R2[Result]
        R3[A] --> R2
        R2 --> RF[Final]
    end
    
    LF -.->|Equivalent| RF
    
    style L1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style R1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style LF fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style RF fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

#### Definition
```
A → B means "A then B"
```

#### Properties
- **Associative**: `(A → B) → C = A → (B → C)`
- **Non-commutative**: `A → B ≠ B → A`
- **Identity**: `ε → A = A → ε = A`

#### Implementation
```rust
impl SubjectSequence {
    pub fn then(self, next: Subject) -> SubjectSequence {
        SubjectSequence {
            subjects: self.subjects.into_iter()
                .chain(std::iter::once(next))
                .collect(),
        }
    }
}
```

#### Use Cases
1. **Workflow Steps**: Order processing → Payment → Shipping
2. **Saga Orchestration**: Each compensation step in sequence
3. **Data Pipeline**: Extract → Transform → Load

### 2. Parallel Composition (⊗)

Parallel composition represents concurrent execution where operations can proceed simultaneously.

#### Parallel Execution Visualization

```mermaid
graph TB
    subgraph "Parallel Composition: A ⊗ B ⊗ C"
        START[Start] 
        A[Subject A<br/>inventory.queries.check]
        B[Subject B<br/>payment.queries.verify]
        C[Subject C<br/>address.queries.validate]
        JOIN[Join Point]
        
        START --> A
        START --> B  
        START --> C
        
        A --> JOIN
        B --> JOIN
        C --> JOIN
    end
    
    subgraph "Execution Timeline"
        T[T0: Start] --> T1[T1: All execute<br/>concurrently]
        T1 --> T2[T2: Fastest completes]
        T2 --> T3[T3: All complete<br/>Join]
    end
    
    style START fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style A fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style B fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style C fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style JOIN fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

#### Commutativity Property

```mermaid
graph LR
    subgraph "A ⊗ B"
        AB1[A] -.->|concurrent| AB2[B]
        AB1 --> ABR[Result]
        AB2 --> ABR
    end
    
    subgraph "B ⊗ A"  
        BA1[B] -.->|concurrent| BA2[A]
        BA1 --> BAR[Result]
        BA2 --> BAR
    end
    
    ABR -.->|Equivalent| BAR
    
    style AB1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style AB2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style BA1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style BA2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style ABR fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style BAR fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

#### Definition
```
A ⊗ B means "A and B concurrently"
```

#### Properties
- **Associative**: `(A ⊗ B) ⊗ C = A ⊗ (B ⊗ C)`
- **Commutative**: `A ⊗ B = B ⊗ A`
- **Identity**: `ε ⊗ A = A ⊗ ε = A`

#### Implementation
```rust
impl SubjectParallel {
    pub fn parallel(self, other: Subject) -> SubjectParallel {
        SubjectParallel {
            subjects: self.subjects.into_iter()
                .chain(std::iter::once(other))
                .collect(),
            join_strategy: JoinStrategy::All,
        }
    }
}
```

#### Use Cases
1. **Fan-out Processing**: Notify multiple services simultaneously
2. **Parallel Validation**: Check inventory ⊗ Verify payment ⊗ Validate address
3. **Scatter-Gather**: Query multiple sources in parallel

### 3. Choice Composition (⊕)

Choice composition represents exclusive alternatives where exactly one path is taken.

#### Choice Flow Visualization

```mermaid
graph TB
    subgraph "Choice Composition: A ⊕ B ⊕ C"
        START[Decision Point]
        A[Subject A<br/>premium.service.process]
        B[Subject B<br/>standard.service.process]  
        C[Subject C<br/>fallback.service.process]
        END[Continue]
        
        START -->|condition 1| A
        START -->|condition 2| B
        START -->|default| C
        
        A --> END
        B --> END
        C --> END
    end
    
    subgraph "Selection Logic"
        COND{Evaluate<br/>Conditions}
        COND -->|Premium User| SEL1[Select A]
        COND -->|Regular User| SEL2[Select B]
        COND -->|Error/Timeout| SEL3[Select C]
    end
    
    style START fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style A fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style B fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style C fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style END fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

#### Exclusive Choice Property

```mermaid
graph LR
    subgraph "Exclusive Selection"
        CH[Choice A ⊕ B]
        CH --> PATH1[Path A Taken]
        CH --> PATH2[Path B Taken]
        
        PATH1 -.->|excludes| PATH2
        PATH2 -.->|excludes| PATH1
    end
    
    subgraph "Execution Result"
        R1[Result if A]
        R2[Result if B]
        RF[Final Result<br/>Either R1 OR R2]
        
        R1 --> RF
        R2 --> RF
    end
    
    PATH1 --> R1
    PATH2 --> R2
    
    style CH fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style PATH1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style PATH2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style RF fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

#### Definition
```
A ⊕ B means "either A or B (but not both)"
```

#### Properties
- **Associative**: `(A ⊕ B) ⊕ C = A ⊕ (B ⊕ C)`
- **Commutative**: `A ⊕ B = B ⊕ A`
- **Identity**: `⊥ ⊕ A = A ⊕ ⊥ = A` (where ⊥ is the zero element)

#### Implementation
```rust
impl SubjectChoice {
    pub fn or(self, alternative: Subject) -> SubjectChoice {
        SubjectChoice {
            alternatives: self.alternatives.into_iter()
                .chain(std::iter::once(alternative))
                .collect(),
            selection_strategy: SelectionStrategy::First,
        }
    }
}
```

#### Use Cases
1. **Conditional Routing**: Premium processing ⊕ Standard processing
2. **Fallback Patterns**: Primary service ⊕ Backup service
3. **A/B Testing**: Route A ⊕ Route B

## Lattice Operations

Subjects form a lattice structure enabling powerful set-theoretic operations.

### Lattice Structure Visualization

```mermaid
graph TD
    subgraph "Subject Lattice"
        TOP[> <br/>Universal Subject]
        
        G[graph.>]
        W[workflow.>]
        
        GE[graph.events.>]
        GC[graph.commands.>]
        
        GES[graph.events.structure.>]
        GEW[graph.events.workflow.>]
        
        GESN[graph.events.structure.node_added]
        GESE[graph.events.structure.edge_added]
        
        BOT[⊥<br/>Empty Subject]
        
        TOP --> G
        TOP --> W
        G --> GE
        G --> GC
        GE --> GES
        GE --> GEW
        GES --> GESN
        GES --> GESE
        GESN --> BOT
        GESE --> BOT
    end
    
    style TOP fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style G fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style GE fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style GES fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style GESN fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style GESE fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style BOT fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
```

### Join Operation (⊔)

The join finds the least upper bound (supremum) of two subjects.

#### Join Operation Visualization

```mermaid
graph TD
    subgraph "Join Operation: A ⊔ B"
        A[graph.events.structure.node_added]
        B[graph.events.structure.edge_added]
        JOIN[graph.events.structure.*<br/>Least Upper Bound]
        
        A --> JOIN
        B --> JOIN
        
        P1[graph.events.structure.>]
        P2[graph.events.>]
        P3[graph.>]
        
        JOIN --> P1
        P1 --> P2
        P2 --> P3
    end
    
    style A fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style B fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style JOIN fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style P1 fill:#E8E8E8,stroke:#999,stroke-width:1px,color:#000
    style P2 fill:#E8E8E8,stroke:#999,stroke-width:1px,color:#000
    style P3 fill:#E8E8E8,stroke:#999,stroke-width:1px,color:#000
```

```rust
pub fn join(a: &Subject, b: &Subject) -> Subject {
    // Find the most specific common ancestor
    let common_prefix = find_common_prefix(a, b);
    Subject::from_tokens(common_prefix)
}
```

Example:
```
graph.events.structure.node_added ⊔ graph.events.structure.edge_added
= graph.events.structure.*
```

### Meet Operation (⊓)

The meet finds the greatest lower bound (infimum) of two subjects.

#### Meet Operation Visualization

```mermaid
graph TD
    subgraph "Meet Operation: A ⊓ B"
        A[graph.events.>]
        B[graph.events.structure.*]
        MEET[graph.events.structure.*<br/>Greatest Lower Bound]
        
        A --> MEET
        B --> MEET
        
        C1[graph.events.structure.node_added]
        C2[graph.events.structure.edge_added]
        
        MEET --> C1
        MEET --> C2
    end
    
    style A fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style B fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style MEET fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style C1 fill:#E8E8E8,stroke:#999,stroke-width:1px,color:#000
    style C2 fill:#E8E8E8,stroke:#999,stroke-width:1px,color:#000
```

```rust
pub fn meet(a: &Subject, b: &Subject) -> Option<Subject> {
    // Find the most general common descendant
    if a.is_parent_of(b) {
        Some(b.clone())
    } else if b.is_parent_of(a) {
        Some(a.clone())
    } else {
        None // No common descendant
    }
}
```

Example:
```
graph.events.> ⊓ graph.events.structure.*
= graph.events.structure.*
```

## Advanced Compositions

### Kleene Star (*)

Represents zero or more repetitions of a subject pattern.

```rust
pub struct KleeneStar {
    base: Subject,
    min_repetitions: usize,
    max_repetitions: Option<usize>,
}

impl KleeneStar {
    pub fn repeat(subject: Subject) -> Self {
        KleeneStar {
            base: subject,
            min_repetitions: 0,
            max_repetitions: None,
        }
    }
}
```

Use case: Retry patterns, recursive workflows

### Subject Transformation

Transform subjects while preserving algebraic properties.

```rust
pub trait SubjectTransform {
    fn map<F>(&self, f: F) -> Subject 
    where 
        F: Fn(&str) -> String;
    
    fn filter<P>(&self, predicate: P) -> Option<Subject>
    where
        P: Fn(&Subject) -> bool;
    
    fn fold<B, F>(&self, init: B, f: F) -> B
    where
        F: Fn(B, &str) -> B;
}
```

## Composition Laws

### Associativity Laws

For sequential composition:
```
(A → B) → C = A → (B → C)
```

For parallel composition:
```
(A ⊗ B) ⊗ C = A ⊗ (B ⊗ C)
```

For choice composition:
```
(A ⊕ B) ⊕ C = A ⊕ (B ⊕ C)
```

### Distribution Laws

Sequential over parallel:
```
A → (B ⊗ C) = (A → B) ⊗ (A → C)  // Only with copying
(A ⊗ B) → C ≠ (A → C) ⊗ (B → C)  // Generally not equal
```

Sequential over choice:
```
A → (B ⊕ C) = (A → B) ⊕ (A → C)
(A ⊕ B) → C = (A → C) ⊕ (B → C)
```

### Identity Laws

For all operations:
```
ε → A = A → ε = A     // Sequential identity
ε ⊗ A = A ⊗ ε = A     // Parallel identity
⊥ ⊕ A = A ⊕ ⊥ = A     // Choice identity (zero element)
```

## Practical Applications

### 1. Workflow Composition

```rust
// Order processing workflow
let workflow = 
    validate_order()
        .then(
            check_inventory()
                .parallel(verify_payment())
                .parallel(validate_shipping_address())
        )
        .then(
            express_shipping()
                .or(standard_shipping())
        )
        .then(send_confirmation());
```

### 2. Event Stream Processing

```rust
// Complex event pattern
let pattern = SubjectPattern::new()
    .starts_with("order.events")
    .then(
        Pattern::any_of(vec![
            "order.events.created",
            "order.events.updated"
        ])
    )
    .within_window(Duration::minutes(5));
```

### 3. Service Mesh Routing

```rust
// Routing with fallbacks
let route = primary_service()
    .or(secondary_service())
    .or(fallback_service())
    .with_timeout(Duration::seconds(30));
```

## Category Theory Connection

### Subjects as a Category

The subject algebra forms a category where:
- **Objects**: Individual subjects
- **Morphisms**: Transformations between subjects
- **Composition**: Sequential composition (→)
- **Identity**: The empty subject transformation

### Functors

Subject patterns can be viewed as functors:
```rust
pub trait SubjectFunctor<F> {
    fn fmap<A, B>(&self, f: F) -> Self
    where 
        F: Fn(A) -> B;
}
```

### Monoid Structure

Subjects under concatenation form a monoid:
- **Binary Operation**: Concatenation
- **Identity Element**: Empty subject
- **Associativity**: `(a · b) · c = a · (b · c)`

## Performance Optimization

### Algebraic Rewriting

Use algebraic laws to optimize subject patterns:

```rust
// Original
(A → B) → (C → D)

// Optimized (associativity)
A → B → C → D

// Original  
A ⊗ A ⊗ B

// Optimized (idempotence for certain operations)
A ⊗ B
```

### Lazy Evaluation

Defer composition until needed:
```rust
pub struct LazySubjectComposition {
    thunk: Box<dyn Fn() -> Subject>,
}

impl LazySubjectComposition {
    pub fn evaluate(&self) -> Subject {
        (self.thunk)()
    }
}
```

### Memoization

Cache frequently used compositions:
```rust
pub struct SubjectCache {
    cache: HashMap<(Subject, Subject, Operation), Subject>,
}
```

## Validation and Verification

### Algebraic Property Testing

```rust
#[cfg(test)]
mod tests {
    use quickcheck::{quickcheck, TestResult};
    
    quickcheck! {
        fn test_sequential_associativity(a: Subject, b: Subject, c: Subject) -> bool {
            (a.then(b)).then(c) == a.then(b.then(c))
        }
        
        fn test_parallel_commutativity(a: Subject, b: Subject) -> bool {
            a.parallel(b) == b.parallel(a)
        }
    }
}
```

### Composition Validation

Ensure valid compositions:
```rust
pub fn validate_composition(comp: &SubjectComposition) -> Result<(), ValidationError> {
    match comp {
        Sequential(a, b) => {
            ensure!(a.can_precede(b), "Invalid sequence");
        }
        Parallel(subjects) => {
            ensure!(!has_conflicts(&subjects), "Conflicting parallel ops");
        }
        Choice(alternatives) => {
            ensure!(alternatives.len() > 1, "Choice needs alternatives");
        }
    }
    Ok(())
}
```

## Future Directions

### Probabilistic Composition

Add weighted choice for A/B testing:
```rust
A ⊕[0.8] B ⊕[0.2] C  // 80% A, 20% B, 0% C
```

### Temporal Composition

Add time constraints:
```rust
A →[≤5s] B  // B must start within 5 seconds of A completing
```

### Conditional Composition

Add guards to compositions:
```rust
A →[if valid] B →[else] C
```