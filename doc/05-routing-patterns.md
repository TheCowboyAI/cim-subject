<!-- Copyright (c) 2025 Cowboy AI, LLC. -->
# Routing Patterns

## Overview

CIM-Subject provides sophisticated routing patterns that enable flexible message delivery in distributed systems. These patterns leverage the hierarchical subject structure, wildcard matching, and correlation tracking to implement complex routing scenarios.

## Routing Architecture Overview

```mermaid
graph TB
    subgraph "Message Flow"
        MSG[Incoming Message]
        SUBJ[Subject:<br/>graph.events.structure.node_added]
        ROUTER{Router}
        
        MSG --> SUBJ
        SUBJ --> ROUTER
    end
    
    subgraph "Routing Strategies"
        DIRECT[Direct Routing<br/>Exact Match]
        PATTERN[Pattern Routing<br/>Wildcards]
        PRIORITY[Priority Routing<br/>Ordered Rules]
        BALANCE[Load Balanced<br/>Distribution]
        
        ROUTER --> DIRECT
        ROUTER --> PATTERN
        ROUTER --> PRIORITY
        ROUTER --> BALANCE
    end
    
    subgraph "Handlers"
        H1[Handler 1]
        H2[Handler 2]
        H3[Handler N]
        
        DIRECT --> H1
        PATTERN --> H2
        PRIORITY --> H3
        BALANCE --> H1
        BALANCE --> H2
        BALANCE --> H3
    end
    
    style MSG fill:#FF6B6B,stroke:#C92A2A,stroke-width:4px,color:#FFF
    style ROUTER fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style DIRECT fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style PATTERN fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style PRIORITY fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style BALANCE fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

## Subject Naming Standard

### Hierarchical Structure Visualization

```mermaid
graph LR
    subgraph "Subject Structure"
        S[Subject] --> D[Domain]
        D --> MT[Message Type]
        MT --> AGG[Aggregate]
        AGG --> ACT[Action]
    end
    
    subgraph "Example: graph.events.structure.node_added"
        EX[graph.events.structure.node_added]
        D1[graph<br/>Domain]
        MT1[events<br/>Type]
        AGG1[structure<br/>Aggregate]
        ACT1[node_added<br/>Action]
        
        EX --> D1
        EX --> MT1
        EX --> AGG1
        EX --> ACT1
    end
    
    subgraph "Semantics"
        D --> SEM1[Bounded Context]
        MT --> SEM2[Message Category]
        AGG --> SEM3[Entity/Service]
        ACT --> SEM4[Operation]
    end
    
    style S fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style D fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style MT fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style AGG fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style ACT fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style EX fill:#2D3436,stroke:#000,stroke-width:2px,color:#FFF
```

The standard subject format follows a strict hierarchy:

```
{domain}.{message_type}.{aggregate}.{action}
```

### Domain Prefixes

Each bounded context has its own domain prefix:

| Domain | Prefix | Purpose |
|--------|---------|----------|
| Graph | `graph` | Graph structure and operations |
| Workflow | `workflow` | Workflow execution and state |
| Location | `location` | Spatial and location services |
| Identity | `identity` | Authentication and authorization |
| Document | `document` | Document management |
| Persistence | `persistence` | Storage operations |
| Intelligence | `intelligence` | AI/ML operations |

### Message Type Segments

Standard message types:
- `commands` - State change requests
- `events` - State change notifications
- `queries` - Read requests
- `responses` - Query responses
- `errors` - Error notifications

### Examples

```
# Commands
graph.commands.structure.add_node
workflow.commands.execution.start
identity.commands.user.authenticate

# Events
graph.events.structure.node_added
workflow.events.execution.completed
document.events.content.uploaded

# Queries
graph.queries.analysis.find_shortest_path
location.queries.search.find_nearby
```

## Wildcard Patterns

### Wildcard Pattern Visualization

```mermaid
graph TB
    subgraph "Single Token Wildcard (*)"
        ST1[graph.events.*.*]
        
        ST1 -->|matches| M1[graph.events.structure.node_added ✓]
        ST1 -->|matches| M2[graph.events.analysis.completed ✓]
        ST1 -->|no match| M3[graph.events.node_added ✗<br/>Too few tokens]
        ST1 -->|no match| M4[graph.commands.structure.add ✗<br/>Wrong message type]
    end
    
    subgraph "Multi Token Wildcard (>)"
        MT1[graph.events.>]
        
        MT1 -->|matches| N1[graph.events.structure ✓]
        MT1 -->|matches| N2[graph.events.structure.node_added ✓]
        MT1 -->|matches| N3[graph.events.analysis.metrics.calculated ✓]
        MT1 -->|no match| N4[graph.commands.structure.add ✗<br/>Wrong prefix]
    end
    
    subgraph "Combined Patterns"
        C1[*.events.user.*]
        
        C1 -->|matches| P1[auth.events.user.created ✓]
        C1 -->|matches| P2[profile.events.user.updated ✓]
        C1 -->|no match| P3[auth.events.user.profile.changed ✗<br/>Too many tokens]
    end
    
    style ST1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style MT1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style C1 fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style M1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style M2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style N1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style N2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style N3 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style P1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style P2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Single Token Wildcard (`*`)

Matches exactly one token:

```rust
pub struct SingleTokenPattern {
    pattern: String,
}

impl SingleTokenPattern {
    pub fn matches(&self, subject: &str) -> bool {
        let pattern_tokens: Vec<&str> = self.pattern.split('.').collect();
        let subject_tokens: Vec<&str> = subject.split('.').collect();
        
        if pattern_tokens.len() != subject_tokens.len() {
            return false;
        }
        
        pattern_tokens.iter()
            .zip(subject_tokens.iter())
            .all(|(p, s)| p == &"*" || p == s)
    }
}
```

Use cases:
```
# Subscribe to all graph events
graph.events.*.* 

# All commands for any aggregate in workflow domain
workflow.commands.*.*

# All user-related events across domains
*.events.user.*
```

### Multi-Token Wildcard (`>`)

Matches one or more tokens at the end:

```rust
pub struct MultiTokenPattern {
    prefix: String,
}

impl MultiTokenPattern {
    pub fn matches(&self, subject: &str) -> bool {
        subject.starts_with(&self.prefix)
    }
}
```

Use cases:
```
# All graph-related messages
graph.>

# All events in the system
*.events.>

# All persistence operations
persistence.>
```

## Routing Strategies

### Routing Strategy Comparison

```mermaid
graph TB
    subgraph "Direct Routing"
        DR[Exact Match Only]
        DR1[graph.events.node_added]
        DR2[→ Handler A]
        DR1 --> DR2
        
        DR3[graph.events.edge_added]
        DR4[→ Handler B]
        DR3 --> DR4
    end
    
    subgraph "Pattern-Based Routing"
        PR[Wildcard Matching]
        PR1[graph.events.*]
        PR2[→ Handlers [A, B, C]]
        PR1 --> PR2
        
        PR3[*.events.>]
        PR4[→ Event Logger]
        PR3 --> PR4
    end
    
    subgraph "Priority-Based Routing"
        PRI[Ordered Evaluation]
        PRI1[Priority 10: graph.events.critical.*]
        PRI2[Priority 5: graph.events.*.*]
        PRI3[Priority 1: graph.>]
        
        PRI1 --> H1[Critical Handler]
        PRI2 --> H2[Normal Handler]
        PRI3 --> H3[Fallback Handler]
    end
    
    subgraph "Load-Balanced Routing"
        LB[Distribution Strategy]
        LB1[Round Robin]
        LB2[Weighted]
        LB3[Least Connections]
        
        LB --> LB1
        LB --> LB2
        LB --> LB3
        
        LB1 --> POOL[Handler Pool<br/>[H1, H2, H3]]
    end
    
    style DR fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style PR fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style PRI fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style LB fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

### 1. Direct Routing

Simple one-to-one subject mapping:

```rust
pub struct DirectRouter {
    routes: HashMap<String, Handler>,
}

impl DirectRouter {
    pub fn route(&self, subject: &str) -> Option<&Handler> {
        self.routes.get(subject)
    }
}
```

### 2. Pattern-Based Routing

Uses wildcards for flexible routing:

```rust
pub struct PatternRouter {
    patterns: Vec<(SubjectPattern, Handler)>,
}

impl PatternRouter {
    pub fn route(&self, subject: &str) -> Vec<&Handler> {
        self.patterns.iter()
            .filter(|(pattern, _)| pattern.matches(subject))
            .map(|(_, handler)| handler)
            .collect()
    }
    
    pub fn add_route(&mut self, pattern: &str, handler: Handler) {
        let pattern = SubjectPattern::parse(pattern);
        self.patterns.push((pattern, handler));
    }
}
```

### 3. Priority-Based Routing

Routes with priority ordering:

```rust
pub struct PriorityRouter {
    routes: BTreeMap<Priority, Vec<(SubjectPattern, Handler)>>,
}

impl PriorityRouter {
    pub fn route(&self, subject: &str) -> Option<&Handler> {
        for (_, patterns) in self.routes.iter().rev() { // Highest priority first
            for (pattern, handler) in patterns {
                if pattern.matches(subject) {
                    return Some(handler);
                }
            }
        }
        None
    }
}
```

### 4. Load-Balanced Routing

#### Load Balancing Visualization

```mermaid
graph TB
    subgraph "Load Balancing Strategies"
        MSG[Incoming Messages<br/>graph.events.>]
        
        subgraph "Round Robin"
            RR[Round Robin<br/>Selector]
            RR1[Handler 1] 
            RR2[Handler 2]
            RR3[Handler 3]
            
            RR -->|1st| RR1
            RR -->|2nd| RR2
            RR -->|3rd| RR3
            RR -->|4th| RR1
        end
        
        subgraph "Weighted Distribution"
            WD[Weighted<br/>Selector]
            WD1[Handler A<br/>Weight: 50%]
            WD2[Handler B<br/>Weight: 30%]
            WD3[Handler C<br/>Weight: 20%]
            
            WD -->|5 of 10| WD1
            WD -->|3 of 10| WD2
            WD -->|2 of 10| WD3
        end
        
        subgraph "Least Connections"
            LC[Connection<br/>Counter]
            LC1[Handler X<br/>Conns: 2]
            LC2[Handler Y<br/>Conns: 5]
            LC3[Handler Z<br/>Conns: 1]
            
            LC -->|Select| LC3
        end
        
        MSG --> RR
        MSG --> WD
        MSG --> LC
    end
    
    style MSG fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style RR fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style WD fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style LC fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style LC3 fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
```

Distributes messages across handlers:

```rust
pub struct LoadBalancedRouter {
    routes: HashMap<SubjectPattern, Vec<Handler>>,
    strategy: LoadBalanceStrategy,
}

pub enum LoadBalanceStrategy {
    RoundRobin,
    Random,
    LeastConnections,
    WeightedRoundRobin(Vec<usize>),
}

impl LoadBalancedRouter {
    pub fn route(&mut self, subject: &str) -> Option<&Handler> {
        for (pattern, handlers) in &mut self.routes {
            if pattern.matches(subject) {
                return self.strategy.select(handlers);
            }
        }
        None
    }
}
```

## Advanced Routing Patterns

### Advanced Pattern Overview

```mermaid
graph TB
    subgraph "Advanced Routing Patterns"
        ADV[Advanced Patterns]
        
        CB[Content-Based<br/>Message inspection]
        SAGA[Saga Routing<br/>Workflow state]
        SG[Scatter-Gather<br/>Fan-out/Fan-in]
        CB2[Circuit Breaker<br/>Fault tolerance]
        
        ADV --> CB
        ADV --> SAGA
        ADV --> SG
        ADV --> CB2
    end
    
    subgraph "Content-Based Example"
        MSG1[Order Message]
        CHECK{Check Amount}
        
        MSG1 --> CHECK
        CHECK -->|> $1000| PREMIUM[Premium Handler]
        CHECK -->|<= $1000| STANDARD[Standard Handler]
    end
    
    subgraph "Saga Example"
        SMSG[Saga Event]
        STATE{Current State}
        
        SMSG --> STATE
        STATE -->|Order Created| PAY[Process Payment]
        STATE -->|Payment Done| SHIP[Ship Order]
        STATE -->|Ship Failed| COMP[Compensate]
    end
    
    subgraph "Scatter-Gather Example"
        REQ[Search Request]
        SCATTER[Scatter to Services]
        S1[Service 1]
        S2[Service 2]
        S3[Service 3]
        GATHER[Gather Results]
        RESP[Combined Response]
        
        REQ --> SCATTER
        SCATTER --> S1
        SCATTER --> S2
        SCATTER --> S3
        S1 --> GATHER
        S2 --> GATHER
        S3 --> GATHER
        GATHER --> RESP
    end
    
    style ADV fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style CB fill:#FF6B6B,stroke:#C92A2A,stroke-width:2px,color:#FFF
    style SAGA fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style SG fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style CB2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### 1. Content-Based Routing

Route based on message content:

```rust
pub struct ContentBasedRouter {
    content_rules: Vec<ContentRule>,
}

pub struct ContentRule {
    subject_pattern: SubjectPattern,
    content_predicate: Box<dyn Fn(&MessageContent) -> bool>,
    handler: Handler,
}

impl ContentBasedRouter {
    pub fn route(&self, message: &Message) -> Option<&Handler> {
        for rule in &self.content_rules {
            if rule.subject_pattern.matches(&message.subject) 
                && (rule.content_predicate)(&message.content) {
                return Some(&rule.handler);
            }
        }
        None
    }
}
```

### 2. Saga Routing

#### Saga State Machine Visualization

```mermaid
stateDiagram-v2
    [*] --> OrderCreated: Start Saga
    
    OrderCreated --> PaymentProcessing: Reserve Payment
    PaymentProcessing --> PaymentReserved: Success
    PaymentProcessing --> PaymentFailed: Failure
    
    PaymentReserved --> InventoryChecking: Check Stock
    InventoryChecking --> InventoryReserved: Available
    InventoryChecking --> InventoryUnavailable: Out of Stock
    
    InventoryReserved --> ShippingScheduled: Schedule Delivery
    ShippingScheduled --> OrderCompleted: Success
    ShippingScheduled --> ShippingFailed: Failure
    
    PaymentFailed --> CompensationStarted: Begin Rollback
    InventoryUnavailable --> ReleasePayment: Compensate
    ShippingFailed --> ReleaseInventory: Compensate
    
    ReleasePayment --> CompensationCompleted
    ReleaseInventory --> ReleasePayment
    CompensationStarted --> CompensationCompleted
    
    OrderCompleted --> [*]: Success
    CompensationCompleted --> [*]: Rolled Back
```

Routes messages within saga workflows:

```rust
pub struct SagaRouter {
    sagas: HashMap<CorrelationId, SagaState>,
}

impl SagaRouter {
    pub fn route(&mut self, message: &Message) -> Result<SagaAction, SagaError> {
        let saga = self.sagas.get_mut(&message.correlation_id)
            .ok_or(SagaError::NotFound)?;
        
        match saga.handle_message(message) {
            SagaTransition::Continue(next_commands) => {
                Ok(SagaAction::SendCommands(next_commands))
            }
            SagaTransition::Complete => {
                self.sagas.remove(&message.correlation_id);
                Ok(SagaAction::Complete)
            }
            SagaTransition::Compensate(compensations) => {
                Ok(SagaAction::Compensate(compensations))
            }
        }
    }
}
```

### 3. Scatter-Gather Routing

#### Scatter-Gather Flow Visualization

```mermaid
graph TB
    subgraph "Scatter Phase"
        REQ[Original Request<br/>Find Best Price]
        CORR[Correlation ID: 123]
        
        REQ --> CORR
        CORR --> SC[Scatter Controller]
        
        SC -->|Clone Request| S1[Supplier 1<br/>Price Service]
        SC -->|Clone Request| S2[Supplier 2<br/>Price Service]
        SC -->|Clone Request| S3[Supplier 3<br/>Price Service]
        SC -->|Clone Request| S4[Supplier 4<br/>Price Service]
    end
    
    subgraph "Gather Phase"
        G[Gatherer<br/>Expects: 4<br/>Timeout: 5s]
        
        S1 -->|Response: $100| G
        S2 -->|Response: $95| G
        S3 -->|Timeout| G
        S4 -->|Response: $110| G
        
        G --> AGG{Aggregation<br/>Strategy}
        AGG -->|Min Price| RESULT[Best Price: $95<br/>From: Supplier 2]
    end
    
    subgraph "Completion"
        RESULT --> RESP[Send Response<br/>to Original Caller]
        G --> CLEAN[Cleanup<br/>Correlation State]
    end
    
    style REQ fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style SC fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style G fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style RESULT fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

Broadcast and collect responses:

```rust
pub struct ScatterGatherRouter {
    scatter_patterns: HashMap<SubjectPattern, Vec<String>>,
    gatherers: HashMap<CorrelationId, Gatherer>,
}

pub struct Gatherer {
    expected_responses: usize,
    received_responses: Vec<Message>,
    completion_handler: Handler,
    timeout: Duration,
}

impl ScatterGatherRouter {
    pub async fn scatter(&mut self, request: &Message) -> Result<(), Error> {
        // Find matching scatter pattern
        let targets = self.find_scatter_targets(&request.subject)?;
        
        // Create gatherer
        let gatherer = Gatherer {
            expected_responses: targets.len(),
            received_responses: Vec::new(),
            completion_handler: self.create_completion_handler(),
            timeout: Duration::from_secs(30),
        };
        
        self.gatherers.insert(request.correlation_id.clone(), gatherer);
        
        // Scatter to all targets
        for target in targets {
            self.send_to_target(target, request).await?;
        }
        
        Ok(())
    }
    
    pub fn gather(&mut self, response: Message) -> Option<Message> {
        if let Some(gatherer) = self.gatherers.get_mut(&response.correlation_id) {
            gatherer.received_responses.push(response);
            
            if gatherer.received_responses.len() == gatherer.expected_responses {
                // All responses received, create aggregate response
                let aggregate = self.aggregate_responses(&gatherer.received_responses);
                self.gatherers.remove(&response.correlation_id);
                return Some(aggregate);
            }
        }
        None
    }
}
```

### 4. Circuit Breaker Routing

#### Circuit Breaker State Transitions

```mermaid
stateDiagram-v2
    [*] --> Closed: Initial State
    
    Closed --> Closed: Success
    Closed --> Closed: Failure < Threshold
    Closed --> Open: Failures >= Threshold
    
    Open --> Open: Requests use Fallback
    Open --> HalfOpen: Timeout Expired
    
    HalfOpen --> Closed: Test Success
    HalfOpen --> Open: Test Failure
    
    note right of Closed
        Normal operation
        Track failures
    end note
    
    note right of Open
        Circuit broken
        Use fallback handler
        Wait for timeout
    end note
    
    note right of HalfOpen
        Test with one request
        Decide next state
    end note
```

#### Circuit Breaker Flow

```mermaid
graph TB
    subgraph "Circuit Breaker Pattern"
        MSG[Incoming Message]
        CB{Circuit State?}
        
        MSG --> CB
        
        CB -->|Closed| PRIMARY[Primary Handler]
        CB -->|Open| FALLBACK[Fallback Handler]
        CB -->|Half-Open| TEST[Test Primary]
        
        PRIMARY -->|Success| SUCCESS[Update Metrics]
        PRIMARY -->|Failure| FAIL[Increment Failures]
        
        FAIL --> CHECK{Threshold<br/>Reached?}
        CHECK -->|Yes| OPEN[Open Circuit]
        CHECK -->|No| CONT[Continue Closed]
        
        TEST -->|Success| RESET[Reset to Closed]
        TEST -->|Failure| REOPEN[Back to Open]
    end
    
    style MSG fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style CB fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style PRIMARY fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style FALLBACK fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style OPEN fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
```

Protects against cascading failures:

```rust
pub struct CircuitBreakerRouter {
    routes: HashMap<SubjectPattern, CircuitBreaker>,
}

pub struct CircuitBreaker {
    primary_handler: Handler,
    fallback_handler: Handler,
    failure_threshold: usize,
    reset_timeout: Duration,
    state: CircuitState,
}

pub enum CircuitState {
    Closed { failure_count: usize },
    Open { opened_at: Instant },
    HalfOpen,
}

impl CircuitBreakerRouter {
    pub fn route(&mut self, message: &Message) -> Result<&Handler, Error> {
        for (pattern, breaker) in &mut self.routes {
            if pattern.matches(&message.subject) {
                return breaker.get_handler();
            }
        }
        Err(Error::NoRoute)
    }
}

impl CircuitBreaker {
    pub fn get_handler(&mut self) -> Result<&Handler, Error> {
        match &self.state {
            CircuitState::Closed { failure_count } => {
                Ok(&self.primary_handler)
            }
            CircuitState::Open { opened_at } => {
                if opened_at.elapsed() > self.reset_timeout {
                    self.state = CircuitState::HalfOpen;
                    Ok(&self.primary_handler)
                } else {
                    Ok(&self.fallback_handler)
                }
            }
            CircuitState::HalfOpen => {
                Ok(&self.primary_handler)
            }
        }
    }
}
```

## Subject Translation

### Subject Translation Flow

```mermaid
graph LR
    subgraph "Internal Context"
        INT[Internal Subject<br/>internal.graph.events.node_added]
    end
    
    subgraph "Translation Layer"
        TRANS{Subject<br/>Translator}
        RULE1[Rule: internal.* → public.*]
        RULE2[Rule: events → notifications]
        RULE3[Rule: v1.* → v2.*]
        
        TRANS --> RULE1
        TRANS --> RULE2
        TRANS --> RULE3
    end
    
    subgraph "External Context"
        EXT[Public Subject<br/>public.graph.notifications.node_added]
    end
    
    subgraph "Version Migration"
        V1[v1.workflow.commands.start]
        V2[v2.workflow.commands.start]
    end
    
    INT --> TRANS
    TRANS --> EXT
    
    V1 --> TRANS
    TRANS --> V2
    
    style INT fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style TRANS fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style EXT fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style V1 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style V2 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

### Context Mapping

Translate subjects between bounded contexts:

```rust
pub struct SubjectTranslator {
    mappings: Vec<TranslationRule>,
}

pub struct TranslationRule {
    source_pattern: SubjectPattern,
    target_template: String,
    transform: Box<dyn Fn(&str) -> String>,
}

impl SubjectTranslator {
    pub fn translate(&self, subject: &str) -> Option<String> {
        for rule in &self.mappings {
            if rule.source_pattern.matches(subject) {
                return Some((rule.transform)(subject));
            }
        }
        None
    }
}

// Example translations
let translator = SubjectTranslator::new()
    .add_rule(
        "internal.graph.events.*",
        "public.graph.notifications.$1",
        |s| s.replace("internal", "public").replace("events", "notifications")
    )
    .add_rule(
        "v1.workflow.commands.*",
        "v2.workflow.commands.$1",
        |s| s.replace("v1", "v2")
    );
```

### Environment Routing

#### Environment-Based Routing Visualization

```mermaid
graph TB
    subgraph "Environment Detection"
        MSG[Message:<br/>graph.events.analysis.completed]
        ENV{Current<br/>Environment}
        
        MSG --> ENV
    end
    
    subgraph "Development Routes"
        DEV[Development]
        DEV1[Debug Handler<br/>Verbose Logging]
        DEV2[Mock Services]
        DEV3[Test Database]
        
        DEV --> DEV1
        DEV --> DEV2
        DEV --> DEV3
    end
    
    subgraph "Staging Routes"
        STAGE[Staging]
        STAGE1[Integration Tests]
        STAGE2[Performance Monitor]
        STAGE3[Staging Database]
        
        STAGE --> STAGE1
        STAGE --> STAGE2
        STAGE --> STAGE3
    end
    
    subgraph "Production Routes"
        PROD[Production]
        PROD1[Optimized Handler]
        PROD2[Load Balancer]
        PROD3[Production Database]
        PROD4[Monitoring/Alerts]
        
        PROD --> PROD1
        PROD --> PROD2
        PROD --> PROD3
        PROD --> PROD4
    end
    
    ENV -->|dev| DEV
    ENV -->|staging| STAGE
    ENV -->|prod| PROD
    
    style MSG fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style ENV fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style DEV fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style STAGE fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style PROD fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
```

Route based on deployment environment:

```rust
pub struct EnvironmentRouter {
    environment: Environment,
    routes: HashMap<Environment, HashMap<SubjectPattern, Handler>>,
}

pub enum Environment {
    Development,
    Staging,
    Production,
}

impl EnvironmentRouter {
    pub fn route(&self, subject: &str) -> Option<&Handler> {
        self.routes.get(&self.environment)
            .and_then(|env_routes| {
                env_routes.iter()
                    .find(|(pattern, _)| pattern.matches(subject))
                    .map(|(_, handler)| handler)
            })
    }
}
```

## Performance Optimization

### Performance Optimization Strategies

```mermaid
graph TB
    subgraph "Subject Index Structure"
        MSG[Incoming Subject<br/>graph.events.structure.node_added]
        
        subgraph "Index Lookup"
            CACHE{Pattern<br/>Cache}
            EXACT[Exact Index<br/>HashMap]
            PREFIX[Prefix Index<br/>BTree]
            PATTERN[Pattern List<br/>Compiled]
            
            MSG --> CACHE
            CACHE -->|Hit| RESULT1[Cached Handlers]
            CACHE -->|Miss| EXACT
            
            EXACT -->|Found| HANDLERS1[Handler Set 1]
            EXACT -->|Not Found| PREFIX
            
            PREFIX -->|Matches| HANDLERS2[Handler Set 2]
            PREFIX -->|Continue| PATTERN
            
            PATTERN -->|Regex Match| HANDLERS3[Handler Set 3]
        end
        
        subgraph "Cache Update"
            HANDLERS1 --> MERGE[Merge Results]
            HANDLERS2 --> MERGE
            HANDLERS3 --> MERGE
            MERGE --> UPDATE[Update Cache]
            UPDATE --> RESULT2[Return Handlers]
        end
    end
    
    style MSG fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style CACHE fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style EXACT fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style PREFIX fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style RESULT1 fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
```

### Subject Index

Pre-compute subject matches for performance:

```rust
pub struct SubjectIndex {
    exact_index: HashMap<String, Vec<Handler>>,
    prefix_index: BTreeMap<String, Vec<Handler>>,
    pattern_cache: LruCache<String, Vec<Handler>>,
}

impl SubjectIndex {
    pub fn lookup(&mut self, subject: &str) -> Vec<&Handler> {
        // Check cache first
        if let Some(handlers) = self.pattern_cache.get(subject) {
            return handlers.iter().collect();
        }
        
        let mut results = Vec::new();
        
        // Exact match
        if let Some(handlers) = self.exact_index.get(subject) {
            results.extend(handlers);
        }
        
        // Prefix match
        for (prefix, handlers) in self.prefix_index.range(..=subject.to_string()) {
            if subject.starts_with(prefix) {
                results.extend(handlers);
            }
        }
        
        // Cache results
        self.pattern_cache.put(subject.to_string(), results.clone());
        
        results.iter().collect()
    }
}
```

### Routing Table Compilation

Compile patterns for faster matching:

```rust
pub struct CompiledRouter {
    routes: Vec<CompiledRoute>,
}

pub struct CompiledRoute {
    pattern: CompiledPattern,
    handler: Handler,
}

pub enum CompiledPattern {
    Exact(String),
    Prefix(String),
    Regex(regex::Regex),
}

impl CompiledPattern {
    pub fn compile(pattern: &str) -> Self {
        if !pattern.contains('*') && !pattern.contains('>') {
            CompiledPattern::Exact(pattern.to_string())
        } else if pattern.ends_with('>') {
            CompiledPattern::Prefix(pattern[..pattern.len()-1].to_string())
        } else {
            let regex_pattern = pattern
                .replace(".", "\\.")
                .replace("*", "[^.]+")
                .replace(">", ".*");
            CompiledPattern::Regex(regex::Regex::new(&regex_pattern).unwrap())
        }
    }
}
```

## Testing Routing

### Route Testing Framework

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_pattern_matching() {
        let pattern = SubjectPattern::new("graph.events.*.*");
        
        assert!(pattern.matches("graph.events.structure.node_added"));
        assert!(pattern.matches("graph.events.analysis.completed"));
        assert!(!pattern.matches("graph.commands.structure.add_node"));
        assert!(!pattern.matches("graph.events.node_added")); // Too few tokens
    }
    
    #[test]
    fn test_router_priority() {
        let mut router = PriorityRouter::new();
        
        router.add_route(Priority::Low, "graph.>", low_handler);
        router.add_route(Priority::High, "graph.events.>", high_handler);
        
        // More specific pattern with higher priority wins
        assert_eq!(
            router.route("graph.events.structure.node_added"),
            Some(&high_handler)
        );
    }
}
```

### Route Coverage Analysis

```rust
pub struct RouteCoverage {
    routes: HashSet<String>,
    covered: HashSet<String>,
}

impl RouteCoverage {
    pub fn record(&mut self, subject: &str) {
        self.covered.insert(subject.to_string());
    }
    
    pub fn coverage_percent(&self) -> f64 {
        (self.covered.len() as f64 / self.routes.len() as f64) * 100.0
    }
    
    pub fn uncovered_routes(&self) -> Vec<&String> {
        self.routes.difference(&self.covered).collect()
    }
}
```