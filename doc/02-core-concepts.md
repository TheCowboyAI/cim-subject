<!-- Copyright (c) 2025 Cowboy AI, LLC. -->
# Core Concepts

## Message Types in CIM

The CIM architecture recognizes three fundamental message types, each serving a specific purpose in the system:

### Message Type Flow Diagram

```mermaid
graph TB
    subgraph "Message Types"
        CMD[Commands]
        EVT[Events]
        QRY[Queries]
    end
    
    subgraph "Command Flow"
        CMD --> VAL{Validate}
        VAL -->|Accept| AGG[Aggregate]
        VAL -->|Reject| ERR[Error Response]
        AGG --> STATE[State Change]
        STATE --> EVTS[Generate Events]
    end
    
    subgraph "Event Flow"
        EVTS --> PUB[Publish]
        PUB --> SUB1[Subscriber 1]
        PUB --> SUB2[Subscriber 2]
        PUB --> SUB3[Subscriber N]
        PUB --> ES[Event Store]
    end
    
    subgraph "Query Flow"
        QRY --> READ[Read Model]
        READ --> PROJ[Projection]
        PROJ --> RESP[Response]
        PROJ --> CACHE[Cache]
    end
    
    style CMD fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style EVT fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style QRY fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style AGG fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style ES fill:#A8E6CF,stroke:#81C784,stroke-width:2px,color:#000
```

### 1. Commands
- **Purpose**: Express intent to change system state
- **Naming Pattern**: `{domain}.commands.{aggregate}.{action}`
- **Characteristics**:
  - Imperative mood (e.g., `CreateUser`, `UpdateLocation`)
  - Can be accepted or rejected
  - Processed by exactly one handler
  - May produce zero or more events

### 2. Events
- **Purpose**: Record facts about state changes
- **Naming Pattern**: `{domain}.events.{aggregate}.{event}`
- **Characteristics**:
  - Past tense (e.g., `UserCreated`, `LocationUpdated`)
  - Immutable once published
  - Can have multiple subscribers
  - Form the event stream for event sourcing

### 3. Queries
- **Purpose**: Request information without changing state
- **Naming Pattern**: `{domain}.queries.{aggregate}.{query}`
- **Characteristics**:
  - Question form (e.g., `GetUserById`, `FindNearbyLocations`)
  - Read-only operations
  - Return data projections
  - Can be cached

## Subject Hierarchy

CIM-Subject uses a hierarchical naming convention that provides semantic meaning and enables flexible routing:

### Subject Structure Visualization

```mermaid
graph LR
    subgraph "Subject Hierarchy"
        ROOT[Subject Root] --> D[Domain]
        D --> MT[Message Type]
        MT --> AGG[Aggregate]
        AGG --> ACT[Action]
    end
    
    subgraph "Example: location.events.location.defined"
        ROOT2[location] --> MT2[events]
        MT2 --> AGG2[location]
        AGG2 --> ACT2[defined]
    end
    
    subgraph "Pattern Matching"
        PAT1[location.*.*.*] -.->|matches| ROOT2
        PAT2[*.events.*.*] -.->|matches| MT2
        PAT3[location.events.>] -.->|matches| AGG2
    end
    
    style D fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style MT fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style AGG fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style ACT fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style ROOT2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style MT2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style AGG2 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style ACT2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

```
{domain}.{message_type}.{aggregate}.{action}
    │         │            │          │
    │         │            │          └─> Specific action/event/query
    │         │            └─> Aggregate root or entity
    │         └─> Type: commands, events, or queries
    └─> Bounded context or domain
```

### Subject Tree Example

```mermaid
graph TD
    ROOT[CIM System]
    
    ROOT --> LOC[location]
    ROOT --> WF[workflow]
    ROOT --> GR[graph]
    
    LOC --> LC[commands]
    LOC --> LE[events]
    LOC --> LQ[queries]
    
    LC --> LCL[location]
    LE --> LEL[location]
    LQ --> LQL[location]
    
    LCL --> LCD[define]
    LCL --> LCU[update]
    LEL --> LED[defined]
    LEL --> LEU[updated]
    LQL --> LQF[find_by_coordinates]
    
    WF --> WC[commands]
    WF --> WE[events]
    
    WC --> WCE[execution]
    WE --> WEE[execution]
    
    WCE --> WCS[start]
    WEE --> WES[started]
    WEE --> WEC[completed]
    
    style ROOT fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style LOC fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style WF fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style GR fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style LC fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style LE fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style LQ fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style WC fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style WE fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
```

### Examples

```
# Location domain
location.commands.location.define
location.events.location.defined
location.queries.location.find_by_coordinates

# Workflow domain
workflow.commands.execution.start
workflow.events.execution.started
workflow.events.execution.completed

# Graph domain
graph.commands.structure.add_node
graph.events.structure.node_added
graph.queries.structure.find_shortest_path
```

### Benefits of Hierarchical Structure

1. **Semantic Clarity**: Subject names convey meaning
2. **Flexible Subscription**: Use wildcards for pattern-based subscriptions
3. **Domain Isolation**: Clear boundaries between contexts
4. **Evolutionary Design**: Easy to extend without breaking existing patterns

## Message Identity

Every message in the CIM system carries a unique identity that enables tracking and correlation:

### Message Identity Trinity

```mermaid
graph TB
    subgraph "Identity Components"
        MI[Message ID<br/>Unique Identifier]
        CI[Correlation ID<br/>Transaction Group]
        CAI[Causation ID<br/>Parent Reference]
    end
    
    subgraph "Root Message"
        R1[Message ID: 123]
        R2[Correlation ID: 123]
        R3[Causation ID: 123]
        R1 -.->|equals| R2
        R2 -.->|equals| R3
    end
    
    subgraph "Derived Message"
        D1[Message ID: 456]
        D2[Correlation ID: 123]
        D3[Causation ID: 123]
        D2 -.->|inherits from parent| R2
        D3 -.->|points to parent| R1
    end
    
    MI --> R1
    CI --> R2
    CAI --> R3
    
    R1 --> D1
    
    style MI fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style CI fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CAI fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style R1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style R2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style R3 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

### MessageIdentity Structure

```rust
pub struct MessageIdentity {
    pub message_id: IdType,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
}
```

### Identity Rules Visualization

```mermaid
graph LR
    subgraph "1. Root Message Pattern"
        ROOT[New Transaction] --> ID1[ID: 123<br/>Corr: 123<br/>Cause: 123]
        ID1 --> NOTE1[All IDs Equal]
    end
    
    subgraph "2. Derived Message Pattern"
        PARENT[Parent Message<br/>ID: 123] --> CHILD[Child Message<br/>ID: 456<br/>Corr: 123<br/>Cause: 123]
        CHILD --> NOTE2[Inherits Correlation]
    end
    
    subgraph "3. Chain Pattern"
        A[Msg A<br/>ID: 123] --> B[Msg B<br/>ID: 456<br/>Cause: 123]
        B --> C[Msg C<br/>ID: 789<br/>Cause: 456]
        A -.->|Correlation: 123| C
    end
    
    style ROOT fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style ID1 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style PARENT fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style CHILD fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style A fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style B fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style C fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

### Transaction Flow Example

```mermaid
sequenceDiagram
    participant User
    participant CMD as Command Handler
    participant AGG as Aggregate
    participant EVT as Event Publisher
    participant SAGA as Saga
    
    User->>CMD: CreateOrder (Root)
    Note over CMD: ID: 123<br/>Corr: 123<br/>Cause: 123
    
    CMD->>AGG: Process Command
    AGG->>EVT: OrderCreated
    Note over EVT: ID: 456<br/>Corr: 123<br/>Cause: 123
    
    EVT->>SAGA: Handle Event
    SAGA->>CMD: ReserveInventory
    Note over CMD: ID: 789<br/>Corr: 123<br/>Cause: 456
    
    rect rgb(255, 235, 205)
        Note over User,SAGA: All messages share Correlation ID: 123
    end
```

### Identity Rules

#### 1. Root Messages (Transaction Start)
When a new business transaction begins:
```
message_id = correlation_id = causation_id
```
This self-referential pattern identifies transaction boundaries.

#### 2. Derived Messages (Within Transaction)
When a message causes another message:
```
new_message.correlation_id = parent.correlation_id  // Inherit correlation
new_message.causation_id = parent.message_id       // Parent caused this
new_message.message_id = <new unique id>           // Fresh identity
```

#### 3. Independent Messages
For messages not part of a transaction:
```
message_id = correlation_id = causation_id
```
Each message forms its own single-message transaction.

## Subject Patterns and Wildcards

CIM-Subject supports NATS-style wildcards for flexible message routing:

### Wildcard Pattern Visualization

```mermaid
graph TB
    subgraph "Pattern Types"
        EXACT[Exact Match<br/>location.events.location.created]
        SINGLE[Single Token *<br/>location.events.*.created]
        MULTI[Multi Token ><br/>location.events.>]
    end
    
    subgraph "Matching Examples"
        S1[location.events.location.created]
        S2[location.events.zone.created]
        S3[location.events.location.zone.created]
        S4[location.events.structure.node.added]
    end
    
    EXACT -.->|matches| S1
    EXACT -.->|no match| S2
    
    SINGLE -.->|matches| S1
    SINGLE -.->|matches| S2
    SINGLE -.->|no match| S3
    
    MULTI -.->|matches| S1
    MULTI -.->|matches| S2
    MULTI -.->|matches| S3
    MULTI -.->|matches| S4
    
    style EXACT fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style SINGLE fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style MULTI fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style S1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style S2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style S3 fill:#A8E6CF,stroke:#81C784,stroke-width:2px,color:#000
    style S4 fill:#A8E6CF,stroke:#81C784,stroke-width:2px,color:#000
```

### Pattern Matching Flow

```mermaid
graph LR
    subgraph "Input"
        MSG[Message Subject<br/>graph.events.structure.node_added]
    end
    
    subgraph "Pattern Matching Engine"
        P1{Pattern:<br/>graph.events.*.*}
        P2{Pattern:<br/>graph.events.>}
        P3{Pattern:<br/>*.events.>}
        P4{Pattern:<br/>graph.commands.>}
    end
    
    subgraph "Results"
        M1[✓ Match]
        M2[✓ Match]
        M3[✓ Match]
        M4[✗ No Match]
    end
    
    MSG --> P1
    MSG --> P2
    MSG --> P3
    MSG --> P4
    
    P1 -->|4 tokens = 4 tokens| M1
    P2 -->|starts with prefix| M2
    P3 -->|token 2 = events| M3
    P4 -->|token 2 ≠ commands| M4
    
    style MSG fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style M1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style M2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style M3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style M4 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
```

### Single Token Wildcard (`*`)
Matches exactly one token in the subject hierarchy.

Examples:
```
location.events.*.created     → Matches: location.events.location.created
                             → Matches: location.events.zone.created
                             → NOT: location.events.location.zone.created

*.commands.user.*            → Matches: auth.commands.user.create
                             → Matches: profile.commands.user.update
```

### Multi-Token Wildcard (`>`)
Matches one or more tokens at the end of the subject.

Examples:
```
graph.events.>               → Matches: graph.events.structure.node_added
                             → Matches: graph.events.workflow.execution.started
                             → Matches: graph.events.lifecycle.created

persistence.>                → Matches: persistence.events.stored
                             → Matches: persistence.commands.backup.start
```

### Pattern Matching Rules

1. Wildcards can appear at any position
2. `*` matches exactly one token
3. `>` can only appear at the end
4. Literal tokens match exactly
5. Case-sensitive matching

## Event Routing Formula

The complete routing key for any message combines subject and identity:

```
EventRoutingKey = Subject + CorrelationId + CausationId
```

This enables sophisticated routing strategies:

1. **Subject-Only Routing**: Traditional pub/sub
2. **Correlation-Based Routing**: All messages in a transaction
3. **Causation-Based Routing**: Follow cause-effect chains
4. **Combined Routing**: Complex patterns using all three

## Domain Boundaries

CIM-Subject helps maintain clear domain boundaries while enabling controlled communication:

### Intra-Domain Communication
Within a single bounded context:
```
location.commands.location.update
    ↓
location.events.location.updated
    ↓
location.projections.location.refresh
```

### Inter-Domain Communication
Between bounded contexts:
```
order.events.order.created
    ↓
inventory.commands.stock.reserve  ← Cross-domain
    ↓
inventory.events.stock.reserved
    ↓
order.commands.order.confirm     ← Back to order domain
```

### Anti-Corruption Layer
Subject translation at domain boundaries:
```
Internal: inventory.events.internal.stock_updated
    ↓ Translation
External: inventory.events.public.inventory_changed
```

## Performance Considerations

### Subject Design for Performance

1. **Keep subjects short**: Reduces parsing overhead
2. **Front-load discrimination**: Put varying parts early
3. **Avoid deep nesting**: Limit to 4-5 levels
4. **Use consistent patterns**: Enables optimization

### Caching Strategies

1. **Pattern Compilation**: Pre-compile wildcard patterns
2. **Route Tables**: Cache subject-to-handler mappings
3. **Identity Indexing**: Index by correlation for fast lookup

### Scalability Patterns

1. **Subject Sharding**: Distribute by subject prefix
2. **Correlation Partitioning**: Group by correlation ID
3. **Temporal Windowing**: Archive old correlations

## Security Implications

### Subject-Based Authorization

Control access at the subject level:
```rust
pub trait SubjectAuthorizer {
    fn can_publish(&self, subject: &str, user: &User) -> bool;
    fn can_subscribe(&self, pattern: &str, user: &User) -> bool;
}
```

### Correlation Isolation

Prevent correlation ID spoofing:
1. Validate correlation chain integrity
2. Restrict correlation ID generation
3. Audit correlation boundaries

### Subject Encryption

For sensitive domains:
1. Encrypt message payloads
2. Use opaque subject names
3. Implement subject-level access control