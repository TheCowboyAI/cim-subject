<!-- Copyright (c) 2025 Cowboy AI, LLC. -->
# Message Identity and Correlation

## Overview

Message identity in CIM-Subject provides a robust mechanism for tracking messages through distributed systems. By maintaining correlation and causation relationships, the system enables complete traceability, distributed debugging, and complex workflow orchestration.

## Identity Architecture Overview

```mermaid
graph TB
    subgraph "Message Identity System"
        MI[Message Identity]
        MI --> MID[Message ID<br/>Unique per message]
        MI --> CID[Correlation ID<br/>Groups transactions]
        MI --> CAID[Causation ID<br/>Links cause-effect]
    end
    
    subgraph "Identity Patterns"
        ROOT[Root Message<br/>Start of transaction]
        DERIVED[Derived Message<br/>Within transaction]
        BRANCH[Branching<br/>Multiple effects]
        
        ROOT --> RPATTERN[All IDs equal]
        DERIVED --> DPATTERN[Inherits correlation<br/>Points to parent]
        BRANCH --> BPATTERN[Multiple children<br/>Same correlation]
    end
    
    subgraph "Applications"
        TRACE[Distributed Tracing]
        AUDIT[Audit Trail]
        DEBUG[Debug Flows]
        SAGA[Saga Orchestration]
    end
    
    MI --> ROOT
    MI --> DERIVED
    MI --> BRANCH
    
    MID --> TRACE
    CID --> SAGA
    CAID --> AUDIT
    CAID --> DEBUG
    
    style MI fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style MID fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CID fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style CAID fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style ROOT fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style DERIVED fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style BRANCH fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

## The Identity Trinity

Every message in CIM carries three essential identifiers that together form its complete identity:

### 1. Message ID
- **Purpose**: Uniquely identifies this specific message
- **Scope**: Global uniqueness across the entire system
- **Lifetime**: Immutable from creation
- **Format**: UUID v4 or similar globally unique identifier

### 2. Correlation ID
- **Purpose**: Groups related messages within a business transaction
- **Scope**: Shared across all messages in a transaction
- **Lifetime**: Propagated throughout the transaction lifecycle
- **Format**: Same as Message ID format

### 3. Causation ID
- **Purpose**: Identifies which message directly caused this one
- **Scope**: Points to a specific Message ID within the same correlation
- **Lifetime**: Set at message creation based on triggering message
- **Format**: Same as Message ID format

## Identity Relationships Visualization

```mermaid
graph LR
    subgraph "Root Message Pattern"
        ROOT[Message<br/>ID: 123<br/>Corr: 123<br/>Cause: 123]
        ROOT --> CHECK1{All IDs Equal?}
        CHECK1 -->|Yes| ISROOT[✓ Root Message]
        CHECK1 -->|No| NOTROOT[✗ Not Root]
    end
    
    subgraph "Derived Message Pattern"
        PARENT[Parent<br/>ID: 123<br/>Corr: ABC<br/>Cause: XYZ]
        CHILD[Child<br/>ID: 456<br/>Corr: ABC<br/>Cause: 123]
        PARENT -->|Creates| CHILD
        CHILD --> INHERIT[Inherits Correlation]
        CHILD --> POINT[Points to Parent]
    end
    
    subgraph "Identity Flow"
        F1[New Transaction] --> F2[Root: ID=Corr=Cause]
        F2 --> F3[Command Processed]
        F3 --> F4[Event: New ID<br/>Same Corr<br/>Cause=Command ID]
        F4 --> F5[Saga: New ID<br/>Same Corr<br/>Cause=Event ID]
    end
    
    style ROOT fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style PARENT fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CHILD fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style F2 fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

## Identity Patterns

### Pattern 1: Root Message (Transaction Initiation)

When starting a new business transaction:

```rust
pub fn create_root_message(payload: MessagePayload) -> Message {
    let id = MessageId::new();
    Message {
        message_id: id.clone(),
        correlation_id: CorrelationId(id.clone()),
        causation_id: CausationId(id),
        payload,
        timestamp: Utc::now(),
    }
}
```

**Characteristics**:
- All three IDs are identical
- Marks the beginning of a transaction
- No parent message exists

**Example**:
```json
{
  "message_id": "550e8400-e29b-41d4-a716-446655440000",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000",
  "causation_id": "550e8400-e29b-41d4-a716-446655440000",
  "type": "CreateOrder",
  "payload": { ... }
}
```

### Pattern 2: Derived Message (Transaction Continuation)

When a message triggers another message:

```rust
pub fn create_derived_message(
    payload: MessagePayload,
    parent: &Message
) -> Message {
    Message {
        message_id: MessageId::new(),
        correlation_id: parent.correlation_id.clone(),
        causation_id: CausationId(parent.message_id.clone()),
        payload,
        timestamp: Utc::now(),
    }
}
```

**Characteristics**:
- New unique Message ID
- Inherits Correlation ID from parent
- Causation ID points to parent's Message ID

**Example**:
```json
{
  "message_id": "6ba7b810-9dad-11d1-80b4-00c04fd430c8",
  "correlation_id": "550e8400-e29b-41d4-a716-446655440000",
  "causation_id": "550e8400-e29b-41d4-a716-446655440000",
  "type": "OrderValidated",
  "payload": { ... }
}
```

### Pattern 3: Branching (Multiple Effects)

When one message causes multiple messages:

```rust
pub fn create_multiple_effects(
    parent: &Message,
    effects: Vec<MessagePayload>
) -> Vec<Message> {
    effects.into_iter()
        .map(|payload| create_derived_message(payload, parent))
        .collect()
}
```

#### Branching Pattern Visualization

```mermaid
graph TB
    subgraph "Branching Message Flow"
        PARENT[Parent Message<br/>ID: A<br/>Corr: 123<br/>Cause: X]
        
        PARENT --> E1[Effect 1<br/>ID: B<br/>Corr: 123<br/>Cause: A]
        PARENT --> E2[Effect 2<br/>ID: C<br/>Corr: 123<br/>Cause: A]
        PARENT --> E3[Effect 3<br/>ID: D<br/>Corr: 123<br/>Cause: A]
        
        E1 --> NOTE1[All share<br/>correlation 123]
        E2 --> NOTE2[All caused by<br/>message A]
        E3 --> NOTE3[Parallel<br/>execution]
    end
    
    subgraph "Identity Inheritance"
        P[Parent Identity] --> C1[Child 1<br/>New ID]
        P --> C2[Child 2<br/>New ID]
        P --> C3[Child 3<br/>New ID]
        
        C1 --> SAME[Same Correlation]
        C2 --> SAME
        C3 --> SAME
    end
    
    style PARENT fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style E1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style E2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style E3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style SAME fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
```

## Correlation Patterns

### Linear Chain Pattern

Most common pattern for sequential processing:

```mermaid
graph LR
    subgraph "Linear Transaction Flow"
        CMD1[Command A<br/>ID: 1<br/>Corr: X<br/>Cause: 1]
        EVT1[Event B<br/>ID: 2<br/>Corr: X<br/>Cause: 1]
        CMD2[Command C<br/>ID: 3<br/>Corr: X<br/>Cause: 2]
        EVT2[Event D<br/>ID: 4<br/>Corr: X<br/>Cause: 3]
        
        CMD1 -->|causes| EVT1
        EVT1 -->|causes| CMD2
        CMD2 -->|causes| EVT2
    end
    
    subgraph "Identity Chain"
        IC1[Message 1] --> IC2[Message 2<br/>Cause: 1]
        IC2 --> IC3[Message 3<br/>Cause: 2]
        IC3 --> IC4[Message 4<br/>Cause: 3]
        
        CORR[Correlation X] -.->|shared by all| IC1
        CORR -.-> IC2
        CORR -.-> IC3
        CORR -.-> IC4
    end
    
    style CMD1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style EVT1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CMD2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style EVT2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CORR fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
```

### Fork-Join Pattern

Parallel processing with synchronization:

```mermaid
graph TB
    subgraph "Fork-Join Pattern"
        A[Message A<br/>Start Process]
        B[Process B<br/>Branch 1]
        C[Process C<br/>Branch 2]
        D[Process D<br/>Branch 3]
        E[Join Point E<br/>Aggregate Results]
        
        A -->|fork| B
        A -->|fork| C
        A -->|fork| D
        
        B -->|join| E
        C -->|join| E
        D -->|join| E
    end
    
    subgraph "Causation Flow"
        CA[Cause: A] --> CB[B caused by A]
        CA --> CC[C caused by A]
        CA --> CD[D caused by A]
        
        CB --> CE1[E caused by B?]
        CC --> CE2[E caused by C?]
        CD --> CE3[E caused by D?]
        
        CE1 --> JS[Join Strategy<br/>Determines Causation]
        CE2 --> JS
        CE3 --> JS
    end
    
    style A fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style B fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style C fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style D fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style E fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style JS fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

### Saga Pattern

Distributed transaction with compensations:

```mermaid
graph TB
    subgraph "Saga Forward Path"
        START[Start Transaction<br/>ID: S]
        S1[Step 1<br/>Order Created]
        S2[Step 2<br/>Payment Reserved]
        S3[Step 3<br/>Inventory Reserved]
        COMPLETE[Complete<br/>All Confirmed]
        
        START --> S1
        S1 --> S2
        S2 --> S3
        S3 --> COMPLETE
    end
    
    subgraph "Compensation Path"
        FAIL[Failure at Step 3]
        C3[Compensate 3<br/>Release Inventory]
        C2[Compensate 2<br/>Refund Payment]
        C1[Compensate 1<br/>Cancel Order]
        COMP[Compensation<br/>Complete]
        
        FAIL --> C3
        C3 --> C2
        C2 --> C1
        C1 --> COMP
    end
    
    S3 -.->|failure| FAIL
    
    subgraph "Causation Tracking"
        F1[Forward: S→1→2→3]
        F2[Reverse: 3→C3→C2→C1]
        F1 -.->|Same Correlation| F2
    end
    
    style START fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style COMPLETE fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style FAIL fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style S1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style S2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style S3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style C3 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style C2 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style C1 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

## Implementation Details

### Message Identity Structure

```rust
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MessageId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CorrelationId(Uuid);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CausationId(Uuid);

pub struct MessageIdentity {
    pub message_id: MessageId,
    pub correlation_id: CorrelationId,
    pub causation_id: CausationId,
}

impl MessageIdentity {
    pub fn new_root() -> Self {
        let id = Uuid::new_v4();
        Self {
            message_id: MessageId(id),
            correlation_id: CorrelationId(id),
            causation_id: CausationId(id),
        }
    }
    
    pub fn new_derived(parent: &MessageIdentity) -> Self {
        Self {
            message_id: MessageId(Uuid::new_v4()),
            correlation_id: parent.correlation_id.clone(),
            causation_id: CausationId(parent.message_id.0),
        }
    }
    
    pub fn is_root(&self) -> bool {
        self.message_id.0 == self.correlation_id.0 
            && self.message_id.0 == self.causation_id.0
    }
}
```

### Causation Tree Builder

#### Causation Tree Visualization

```mermaid
graph TB
    subgraph "Causation Tree Structure"
        ROOT[Root Message<br/>ID: R1]
        
        ROOT --> C1[Command 1<br/>ID: C1<br/>Cause: R1]
        ROOT --> C2[Command 2<br/>ID: C2<br/>Cause: R1]
        
        C1 --> E1[Event 1<br/>ID: E1<br/>Cause: C1]
        C1 --> E2[Event 2<br/>ID: E2<br/>Cause: C1]
        
        C2 --> E3[Event 3<br/>ID: E3<br/>Cause: C2]
        
        E1 --> S1[Saga Step 1<br/>ID: S1<br/>Cause: E1]
        E3 --> S2[Saga Step 2<br/>ID: S2<br/>Cause: E3]
        
        S1 --> COMP1[Complete 1<br/>ID: CP1<br/>Cause: S1]
        S2 --> COMP2[Complete 2<br/>ID: CP2<br/>Cause: S2]
    end
    
    subgraph "Tree Navigation"
        NAV1[Get Path to Root] --> PATH[R1 → C1 → E1 → S1]
        NAV2[Get Children] --> CHILD[C1 → [E1, E2]]
        NAV3[Get Depth] --> DEPTH[S1 = Depth 3]
    end
    
    style ROOT fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style C1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style C2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style E1 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style E2 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style E3 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style S1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style S2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

```rust
pub struct CausationTree {
    nodes: HashMap<MessageId, TreeNode>,
    roots: Vec<MessageId>,
}

struct TreeNode {
    message: Message,
    children: Vec<MessageId>,
}

impl CausationTree {
    pub fn build(messages: Vec<Message>) -> Self {
        let mut tree = CausationTree {
            nodes: HashMap::new(),
            roots: Vec::new(),
        };
        
        // First pass: create nodes
        for msg in messages {
            let node = TreeNode {
                message: msg.clone(),
                children: Vec::new(),
            };
            tree.nodes.insert(msg.message_id.clone(), node);
            
            // Identify roots
            if msg.is_root() {
                tree.roots.push(msg.message_id.clone());
            }
        }
        
        // Second pass: build parent-child relationships
        for (id, node) in &tree.nodes {
            if !node.message.is_root() {
                if let Some(parent) = tree.nodes.get_mut(&node.message.causation_id.0) {
                    parent.children.push(id.clone());
                }
            }
        }
        
        tree
    }
    
    pub fn get_path_to_root(&self, message_id: &MessageId) -> Vec<Message> {
        let mut path = Vec::new();
        let mut current = Some(message_id);
        
        while let Some(id) = current {
            if let Some(node) = self.nodes.get(id) {
                path.push(node.message.clone());
                
                if node.message.is_root() {
                    break;
                }
                
                current = Some(&MessageId(node.message.causation_id.0));
            } else {
                break;
            }
        }
        
        path.reverse();
        path
    }
}
```

## NATS Integration

### NATS Header Flow Visualization

```mermaid
graph LR
    subgraph "Message Creation"
        MSG[Message<br/>with Identity]
        ID[Message ID: 123]
        CORR[Correlation ID: ABC]
        CAUS[Causation ID: XYZ]
        
        MSG --> ID
        MSG --> CORR
        MSG --> CAUS
    end
    
    subgraph "NATS Headers"
        H1[X-Message-ID: 123]
        H2[X-Correlation-ID: ABC]
        H3[X-Causation-ID: XYZ]
        
        ID --> H1
        CORR --> H2
        CAUS --> H3
    end
    
    subgraph "NATS Transport"
        PUB[Publisher] --> HEADERS[Headers + Payload]
        HEADERS --> BUS[NATS Bus]
        BUS --> SUB[Subscriber]
    end
    
    subgraph "Message Reception"
        SUB --> PARSE[Parse Headers]
        PARSE --> REBUILD[Rebuild Identity]
        REBUILD --> NEWMSG[Message<br/>with Identity]
    end
    
    H1 --> HEADERS
    H2 --> HEADERS
    H3 --> HEADERS
    
    style MSG fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style HEADERS fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style BUS fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style NEWMSG fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

### Header Mapping

CIM-Subject automatically maps identity to NATS headers:

```rust
impl NatsHeaders for MessageIdentity {
    fn to_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert("X-Message-ID", self.message_id.to_string());
        headers.insert("X-Correlation-ID", self.correlation_id.to_string());
        headers.insert("X-Causation-ID", self.causation_id.to_string());
        headers
    }
    
    fn from_headers(headers: &HeaderMap) -> Result<Self, ParseError> {
        Ok(Self {
            message_id: MessageId(parse_uuid(headers.get("X-Message-ID")?)?),
            correlation_id: CorrelationId(parse_uuid(headers.get("X-Correlation-ID")?)?),
            causation_id: CausationId(parse_uuid(headers.get("X-Causation-ID")?)?),
        })
    }
}
```

### Publishing with Identity

```rust
pub async fn publish_with_identity(
    client: &NatsClient,
    subject: &str,
    payload: &[u8],
    identity: &MessageIdentity,
) -> Result<(), PublishError> {
    let headers = identity.to_headers();
    client.publish_with_headers(subject, headers, payload).await
}
```

## Correlation-Based Routing

### Correlation Routing Flow

```mermaid
graph TB
    subgraph "Correlation-Based Routing"
        MSG1[Message 1<br/>Corr: ABC]
        MSG2[Message 2<br/>Corr: ABC]
        MSG3[Message 3<br/>Corr: XYZ]
        MSG4[Message 4<br/>Corr: ABC]
        
        ROUTER{Correlation<br/>Router}
        
        MSG1 --> ROUTER
        MSG2 --> ROUTER
        MSG3 --> ROUTER
        MSG4 --> ROUTER
        
        ROUTER -->|ABC| H1[Handler A<br/>Transaction ABC]
        ROUTER -->|XYZ| H2[Handler B<br/>Transaction XYZ]
    end
    
    subgraph "Dynamic Routing Table"
        TABLE[Route Table]
        R1[ABC → Handler A]
        R2[XYZ → Handler B]
        R3[DEF → Handler C]
        
        TABLE --> R1
        TABLE --> R2
        TABLE --> R3
    end
    
    subgraph "Subscription Pattern"
        SUB1[Subscribe to<br/>Correlation ABC]
        SUB2[Filter:<br/>X-Correlation-ID=ABC]
        SUB3[Receive all<br/>ABC messages]
        
        SUB1 --> SUB2
        SUB2 --> SUB3
    end
    
    style MSG1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style MSG2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style MSG4 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style MSG3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style ROUTER fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style H1 fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style H2 fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

### Route by Correlation

Route all messages in a transaction to specific handlers:

```rust
pub struct CorrelationRouter {
    routes: HashMap<CorrelationId, Handler>,
}

impl CorrelationRouter {
    pub fn route(&self, message: &Message) -> Option<&Handler> {
        self.routes.get(&message.correlation_id)
    }
    
    pub fn route_transaction(&mut self, correlation_id: CorrelationId, handler: Handler) {
        self.routes.insert(correlation_id, handler);
    }
}
```

### Correlation-Based Subscriptions

Subscribe to all messages in a correlation:

```rust
pub async fn subscribe_to_correlation(
    client: &NatsClient,
    correlation_id: &CorrelationId,
) -> Result<Subscription, SubscribeError> {
    // Use NATS headers subscription
    let subject = ">";  // All subjects
    let filter = HeaderFilter::new()
        .add("X-Correlation-ID", correlation_id.to_string());
    
    client.subscribe_with_filter(subject, filter).await
}
```

## Distributed Tracing

### Distributed Trace Visualization

```mermaid
graph TB
    subgraph "Service A"
        A1[Request Received<br/>Corr: 123]
        A2[Create Span A<br/>Trace: 123<br/>Span: A1]
        A3[Process]
        A4[Call Service B]
        
        A1 --> A2
        A2 --> A3
        A3 --> A4
    end
    
    subgraph "Service B"
        B1[Request from A<br/>Parent: A1]
        B2[Create Span B<br/>Trace: 123<br/>Span: B1<br/>Parent: A1]
        B3[Process]
        B4[Call Service C]
        
        B1 --> B2
        B2 --> B3
        B3 --> B4
    end
    
    subgraph "Service C"
        C1[Request from B<br/>Parent: B1]
        C2[Create Span C<br/>Trace: 123<br/>Span: C1<br/>Parent: B1]
        C3[Process]
        C4[Return Result]
        
        C1 --> C2
        C2 --> C3
        C3 --> C4
    end
    
    A4 -.->|Headers| B1
    B4 -.->|Headers| C1
    
    subgraph "Trace Tree"
        TRACE[Trace 123]
        SPAN1[Span A1<br/>Service A]
        SPAN2[Span B1<br/>Service B]
        SPAN3[Span C1<br/>Service C]
        
        TRACE --> SPAN1
        SPAN1 --> SPAN2
        SPAN2 --> SPAN3
    end
    
    style A1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style B1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style C1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style TRACE fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

### Trace Context Propagation

```rust
pub struct TraceContext {
    pub correlation_id: CorrelationId,
    pub span_id: SpanId,
    pub trace_flags: TraceFlags,
}

impl From<&MessageIdentity> for TraceContext {
    fn from(identity: &MessageIdentity) -> Self {
        TraceContext {
            correlation_id: identity.correlation_id.clone(),
            span_id: SpanId::from_message_id(&identity.message_id),
            trace_flags: TraceFlags::default(),
        }
    }
}
```

### Integration with OpenTelemetry

```rust
pub fn message_to_span(message: &Message) -> Span {
    let span = tracer::span_builder(&message.subject)
        .with_trace_id(message.correlation_id.to_trace_id())
        .with_span_id(message.message_id.to_span_id())
        .with_parent_span_id(message.causation_id.to_span_id())
        .start();
    
    span.set_attribute("message.type", message.message_type());
    span.set_attribute("correlation.id", message.correlation_id.to_string());
    span
}
```

## Event Sourcing Integration

### Event Store Organization

```mermaid
graph TB
    subgraph "Event Stream Organization"
        EVT1[Event 1<br/>ID: E1<br/>Corr: ABC<br/>Cause: R1]
        EVT2[Event 2<br/>ID: E2<br/>Corr: ABC<br/>Cause: E1]
        EVT3[Event 3<br/>ID: E3<br/>Corr: XYZ<br/>Cause: R2]
        EVT4[Event 4<br/>ID: E4<br/>Corr: ABC<br/>Cause: E2]
        
        STORE{Event Store}
        
        EVT1 --> STORE
        EVT2 --> STORE
        EVT3 --> STORE
        EVT4 --> STORE
    end
    
    subgraph "Correlation Index"
        CIDX[By Correlation]
        ABC[ABC: [E1, E2, E4]]
        XYZ[XYZ: [E3]]
        
        CIDX --> ABC
        CIDX --> XYZ
    end
    
    subgraph "Causation Index"
        CAIDX[By Causation]
        R1IDX[R1: [E1]]
        E1IDX[E1: [E2]]
        R2IDX[R2: [E3]]
        E2IDX[E2: [E4]]
        
        CAIDX --> R1IDX
        CAIDX --> E1IDX
        CAIDX --> R2IDX
        CAIDX --> E2IDX
    end
    
    STORE --> CIDX
    STORE --> CAIDX
    
    subgraph "Query Patterns"
        Q1[Get Transaction<br/>History] --> ABC
        Q2[Get Event<br/>Effects] --> E1IDX
        Q3[Replay in<br/>Order] --> TREE[Build Causation Tree]
    end
    
    style EVT1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style EVT2 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style EVT4 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style EVT3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style STORE fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style ABC fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

### Event Stream Organization

```rust
pub struct EventStore {
    // Events indexed by correlation
    by_correlation: HashMap<CorrelationId, Vec<Event>>,
    // Events indexed by causation
    by_causation: HashMap<CausationId, Vec<Event>>,
}

impl EventStore {
    pub fn append(&mut self, event: Event) {
        // Index by correlation
        self.by_correlation
            .entry(event.correlation_id.clone())
            .or_default()
            .push(event.clone());
        
        // Index by causation
        self.by_causation
            .entry(event.causation_id.clone())
            .or_default()
            .push(event);
    }
    
    pub fn get_transaction_history(&self, correlation_id: &CorrelationId) -> Vec<Event> {
        self.by_correlation
            .get(correlation_id)
            .cloned()
            .unwrap_or_default()
    }
}
```

### Replay with Causation Order

#### Causal Order Replay Visualization

```mermaid
graph TB
    subgraph "Original Event Order"
        O1[E4: Time 16:04]
        O2[E1: Time 16:01]
        O3[E3: Time 16:03]
        O4[E2: Time 16:02]
        
        O1 --> O2
        O2 --> O3
        O3 --> O4
    end
    
    subgraph "Causation Analysis"
        T1[Build Tree]
        T2[Find Roots:<br/>E1 (R1→E1)]
        T3[Trace Chains:<br/>E1→E2→E4<br/>E3 (standalone)]
        
        T1 --> T2
        T2 --> T3
    end
    
    subgraph "Causal Order Result"
        R1[E1: First<br/>Root Event]
        R2[E2: Second<br/>Caused by E1]
        R3[E4: Third<br/>Caused by E2]
        R4[E3: Fourth<br/>Independent Root]
        
        R1 --> R2
        R2 --> R3
        R3 --> R4
    end
    
    O4 -.->|Reorder| T1
    T3 -.->|Output| R1
    
    style O1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style O2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style O3 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style O4 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style R1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style R2 fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style R3 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style R4 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
```

```rust
pub fn replay_in_causal_order(events: Vec<Event>) -> Vec<Event> {
    let tree = CausationTree::build(events);
    let mut ordered = Vec::new();
    
    // Depth-first traversal ensures causal order
    for root in &tree.roots {
        depth_first_traverse(&tree, root, &mut ordered);
    }
    
    ordered
}
```

## Best Practices

### 1. Always Propagate Correlation

```rust
// Good
let response = create_derived_message(ResponsePayload { ... }, &request);

// Bad
let response = create_root_message(ResponsePayload { ... }); // Loses correlation!
```

### 2. Validate Identity Consistency

```rust
pub fn validate_identity(message: &Message) -> Result<(), ValidationError> {
    // Check root message consistency
    if message.is_root() {
        ensure!(
            message.message_id == message.correlation_id 
            && message.message_id == message.causation_id,
            "Root message must have all IDs equal"
        );
    }
    
    // Check that IDs are valid UUIDs
    ensure!(message.message_id.is_valid(), "Invalid message ID");
    ensure!(message.correlation_id.is_valid(), "Invalid correlation ID");
    ensure!(message.causation_id.is_valid(), "Invalid causation ID");
    
    Ok(())
}
```

### 3. Use Correlation for Business Transactions

#### Transaction Boundary Visualization

```mermaid
graph TB
    subgraph "Order Transaction Boundary"
        START[Start Order<br/>Transaction]
        CORR[Generate<br/>Correlation ID: 123]
        
        START --> CORR
        
        subgraph "Transaction Scope"
            CMD1[Create Order<br/>Corr: 123]
            EVT1[Order Created<br/>Corr: 123]
            CMD2[Reserve Payment<br/>Corr: 123]
            EVT2[Payment Reserved<br/>Corr: 123]
            CMD3[Allocate Stock<br/>Corr: 123]
            EVT3[Stock Allocated<br/>Corr: 123]
            
            CMD1 --> EVT1
            EVT1 --> CMD2
            CMD2 --> EVT2
            EVT2 --> CMD3
            CMD3 --> EVT3
        end
        
        CORR --> CMD1
        EVT3 --> END[Transaction<br/>Complete]
    end
    
    subgraph "Outside Transaction"
        OTHER1[Other Order<br/>Corr: 456]
        OTHER2[Analytics Query<br/>Corr: 789]
        
        OTHER1 -.->|Different Correlation| CMD1
        OTHER2 -.->|No Interference| EVT2
    end
    
    style START fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style CORR fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style END fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style CMD1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style CMD2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style CMD3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
```

```rust
pub struct OrderTransaction {
    correlation_id: CorrelationId,
    commands: Vec<Command>,
    events: Vec<Event>,
}

impl OrderTransaction {
    pub fn new() -> Self {
        Self {
            correlation_id: CorrelationId(Uuid::new_v4()),
            commands: Vec::new(),
            events: Vec::new(),
        }
    }
    
    pub fn execute_command(&mut self, command: Command) -> Result<(), Error> {
        // Ensure command uses transaction's correlation
        let command = command.with_correlation(self.correlation_id.clone());
        
        // Process command...
        self.commands.push(command);
        Ok(())
    }
}
```

## Troubleshooting

### Common Issues

1. **Broken Correlation Chains**
   - Symptom: Related messages have different correlation IDs
   - Cause: Creating new root messages instead of derived
   - Fix: Always use `create_derived_message` within transactions

2. **Circular Causation**
   - Symptom: Infinite loops in causation trees
   - Cause: Message A causes B, B causes A
   - Fix: Validate causation chains, prevent cycles

3. **Orphaned Messages**
   - Symptom: Messages with causation IDs that don't exist
   - Cause: Messages processed out of order or lost
   - Fix: Implement proper message ordering and persistence

### Debugging Tools

```rust
pub struct CorrelationDebugger {
    pub fn print_transaction_flow(&self, correlation_id: &CorrelationId) {
        let messages = self.store.get_by_correlation(correlation_id);
        let tree = CausationTree::build(messages);
        
        println!("Transaction: {}", correlation_id);
        for root in &tree.roots {
            self.print_tree(&tree, root, 0);
        }
    }
    
    fn print_tree(&self, tree: &CausationTree, node_id: &MessageId, depth: usize) {
        let indent = "  ".repeat(depth);
        if let Some(node) = tree.nodes.get(node_id) {
            println!("{}{}: {}", indent, node.message.message_type(), node_id);
            for child in &node.children {
                self.print_tree(tree, child, depth + 1);
            }
        }
    }
}
```