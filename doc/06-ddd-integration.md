<!-- Copyright (c) 2025 Cowboy AI, LLC. -->
# Domain-Driven Design Integration

## Overview

CIM-Subject provides first-class support for Domain-Driven Design (DDD) patterns, enabling proper bounded context separation, aggregate communication, and event-driven architectures. This integration ensures that domain concepts remain pure while leveraging powerful messaging infrastructure.

## DDD Architecture with CIM-Subject

```mermaid
graph TB
    subgraph "Bounded Contexts"
        subgraph "Location Context"
            LA[Location Aggregate]
            LC[Location Commands]
            LE[Location Events]
            LQ[Location Queries]
        end
        
        subgraph "Workflow Context"
            WA[Workflow Aggregate]
            WC[Workflow Commands]
            WE[Workflow Events]
            WQ[Workflow Queries]
        end
        
        subgraph "Identity Context"
            IA[Identity Aggregate]
            IC[Identity Commands]
            IE[Identity Events]
            IQ[Identity Queries]
        end
    end
    
    subgraph "CIM-Subject Layer"
        ROUTER[Subject Router]
        CORR[Correlation Tracker]
        ACL[Anti-Corruption Layer]
    end
    
    subgraph "Infrastructure"
        NATS[NATS Bus]
        ES[Event Store]
        PROJ[Projections]
    end
    
    LC --> ROUTER
    WC --> ROUTER
    IC --> ROUTER
    
    ROUTER --> NATS
    NATS --> ES
    ES --> PROJ
    
    LE -.->|Translated| ACL
    ACL -.->|Public Events| NATS
    
    style LA fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style WA fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style IA fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style ROUTER fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style ACL fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style NATS fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

## Bounded Context Communication

### Context Boundaries Visualization

```mermaid
graph LR
    subgraph "Order Context"
        O1[Order Domain]
        O2[Order Language]
        O3[Order Rules]
    end
    
    subgraph "Inventory Context"
        I1[Inventory Domain]
        I2[Inventory Language]
        I3[Stock Rules]
    end
    
    subgraph "Context Mapping"
        CM[Context Map]
        ACL1[Order→Inventory ACL]
        ACL2[Inventory→Order ACL]
        
        CM --> ACL1
        CM --> ACL2
    end
    
    subgraph "Subject Translation"
        ST1[order.events.created]
        ST2[→ inventory.commands.reserve]
        
        ST3[inventory.events.reserved]
        ST4[→ order.commands.confirm]
    end
    
    O1 -.->|Publishes| ST1
    ST1 --> ACL1
    ACL1 --> ST2
    ST2 --> I1
    
    I1 -.->|Publishes| ST3
    ST3 --> ACL2
    ACL2 --> ST4
    ST4 --> O1
    
    style O1 fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style I1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CM fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style ACL1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style ACL2 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Context Mapping

CIM-Subject enforces clear boundaries between bounded contexts through subject namespacing:

```rust
pub trait BoundedContext {
    fn name(&self) -> &str;
    fn subject_prefix(&self) -> String {
        self.name().to_lowercase()
    }
}

pub struct LocationContext;
impl BoundedContext for LocationContext {
    fn name(&self) -> &str { "Location" }
}

pub struct WorkflowContext;
impl BoundedContext for WorkflowContext {
    fn name(&self) -> &str { "Workflow" }
}
```

### Anti-Corruption Layer

#### ACL Translation Flow

```mermaid
graph TB
    subgraph "External Interface"
        EXT[External Request<br/>REST/GraphQL]
        EXTDATA[External Data Model<br/>{ place_name, lat, lng }]
    end
    
    subgraph "Anti-Corruption Layer"
        VAL{Validate}
        TRANS[Translate]
        MAP[Map Fields]
        ENRICH[Enrich Data]
        
        EXT --> VAL
        VAL -->|Valid| TRANS
        VAL -->|Invalid| REJECT[Reject]
        TRANS --> MAP
        MAP --> ENRICH
    end
    
    subgraph "Internal Domain"
        CMD[Domain Command<br/>DefineLocation]
        AGG[Location Aggregate]
        EVT[Domain Event<br/>LocationDefined]
    end
    
    subgraph "Outbound Translation"
        EVTTRANS[Event Translator]
        PUBEVT[Public Event<br/>LocationCreated]
        NOTIFY[External Notification]
    end
    
    ENRICH --> CMD
    CMD --> AGG
    AGG --> EVT
    EVT --> EVTTRANS
    EVTTRANS --> PUBEVT
    PUBEVT --> NOTIFY
    
    style EXT fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style VAL fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style TRANS fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style CMD fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style AGG fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

Translate between internal and external representations:

```rust
pub trait AntiCorruptionLayer {
    type InternalCommand;
    type ExternalCommand;
    type InternalEvent;
    type ExternalEvent;
    
    fn translate_inbound_command(&self, external: Self::ExternalCommand) -> Result<Self::InternalCommand>;
    fn translate_outbound_event(&self, internal: Self::InternalEvent) -> Self::ExternalEvent;
}

pub struct LocationACL;

impl AntiCorruptionLayer for LocationACL {
    type InternalCommand = DefineLocation;
    type ExternalCommand = ExternalLocationRequest;
    type InternalEvent = LocationDefined;
    type ExternalEvent = LocationCreatedNotification;
    
    fn translate_inbound_command(&self, external: ExternalLocationRequest) -> Result<DefineLocation> {
        Ok(DefineLocation {
            location_id: Uuid::new_v4(),
            name: external.place_name,
            location_type: self.map_location_type(external.category),
            coordinates: self.map_coordinates(external.lat, external.lng),
            // ... additional mapping
        })
    }
    
    fn translate_outbound_event(&self, internal: LocationDefined) -> LocationCreatedNotification {
        LocationCreatedNotification {
            place_id: internal.location_id.to_string(),
            place_name: internal.name,
            created_at: Utc::now(),
            // ... additional mapping
        }
    }
}
```

## Aggregate Patterns

### Aggregate Command Flow

```mermaid
sequenceDiagram
    participant Client
    participant Handler as Command Handler
    participant Repo as Repository
    participant Agg as Aggregate
    participant ES as Event Store
    participant Pub as Event Publisher
    
    Client->>Handler: Command Envelope<br/>[ID, Correlation, Command]
    Handler->>Repo: Load Aggregate
    
    alt Aggregate Exists
        Repo->>ES: Load Events
        ES->>Repo: Event Stream
        Repo->>Agg: Rebuild from Events
        Repo->>Handler: Aggregate
    else New Aggregate
        Repo->>Handler: None
        Handler->>Agg: Create New
    end
    
    Handler->>Agg: Handle Command
    Agg->>Agg: Validate Rules
    Agg->>Agg: Generate Events
    Agg->>Handler: Events/Error
    
    alt Success
        Handler->>ES: Save Events
        Handler->>Pub: Publish Events<br/>[With Correlation]
        Handler->>Client: Acknowledgment<br/>[Accepted]
    else Failure
        Handler->>Client: Acknowledgment<br/>[Rejected]
    end
```

### Aggregate State Machine

```mermaid
stateDiagram-v2
    [*] --> Created: Define Command
    
    Created --> Active: Activate
    Created --> [*]: Delete
    
    Active --> Updated: Update Command
    Active --> Suspended: Suspend
    Active --> [*]: Delete
    
    Updated --> Active: More Updates
    
    Suspended --> Active: Resume
    Suspended --> [*]: Delete
    
    note right of Active
        Most commands handled
        in Active state
    end note
    
    note right of Suspended
        Limited commands
        allowed
    end note
```

### Command Handling

Integrate CIM-Subject with aggregate command handling:

```rust
pub trait Aggregate {
    type Command;
    type Event;
    type Error;
    
    fn handle_command(&mut self, command: Self::Command) -> Result<Vec<Self::Event>, Self::Error>;
}

pub struct CommandEnvelope<C> {
    pub id: CommandId,
    pub identity: MessageIdentity,
    pub command: C,
    pub issued_by: String,
}

pub trait CommandHandler<C> {
    fn handle(&mut self, envelope: CommandEnvelope<C>) -> CommandAcknowledgment;
}

// Example: Location aggregate command handler
pub struct LocationCommandHandler<R: AggregateRepository<Location>> {
    repository: Arc<R>,
    event_publisher: Arc<dyn EventPublisher>,
}

impl<R: AggregateRepository<Location>> CommandHandler<DefineLocation> for LocationCommandHandler<R> {
    fn handle(&mut self, envelope: CommandEnvelope<DefineLocation>) -> CommandAcknowledgment {
        let command = &envelope.command;
        let location_id = EntityId::from_uuid(command.location_id);
        
        // Load or create aggregate
        let result = match self.repository.load(location_id) {
            Ok(Some(_)) => {
                CommandAcknowledgment::rejected(
                    envelope.id,
                    envelope.identity.correlation_id.clone(),
                    "Location already exists"
                )
            }
            Ok(None) => {
                // Create new location aggregate
                let location = Location::define(
                    location_id,
                    command.name.clone(),
                    command.location_type.clone(),
                    command.coordinates.clone(),
                )?;
                
                // Save aggregate
                self.repository.save(&location)?;
                
                // Publish events with correlation
                let events = vec![LocationDefined {
                    location_id: command.location_id,
                    name: command.name.clone(),
                    location_type: command.location_type.clone(),
                    // ...
                }];
                
                self.event_publisher.publish_events(
                    events,
                    envelope.identity.correlation_id.clone()
                )?;
                
                CommandAcknowledgment::accepted(
                    envelope.id,
                    envelope.identity.correlation_id.clone()
                )
            }
            Err(e) => {
                CommandAcknowledgment::rejected(
                    envelope.id,
                    envelope.identity.correlation_id.clone(),
                    &format!("Repository error: {}", e)
                )
            }
        };
        
        result
    }
}
```

### Event Publishing

#### Event Publishing Flow

```mermaid
graph TB
    subgraph "Aggregate Processing"
        CMD[Command Received]
        AGG[Aggregate<br/>Processes Command]
        EVTS[Events Generated<br/>[E1, E2, E3]]
        
        CMD --> AGG
        AGG --> EVTS
    end
    
    subgraph "Event Publishing"
        PUB{Event Publisher}
        MAPPER[Subject Mapper]
        IDENT[Identity Creator]
        
        EVTS --> PUB
        PUB --> MAPPER
        PUB --> IDENT
        
        MAPPER --> S1[location.events.location.defined]
        MAPPER --> S2[location.events.metadata.updated]
        MAPPER --> S3[location.events.tags.added]
        
        IDENT --> ID1[Derived Identity<br/>Same Correlation]
        IDENT --> ID2[Derived Identity<br/>Same Correlation]
        IDENT --> ID3[Derived Identity<br/>Same Correlation]
    end
    
    subgraph "NATS Publishing"
        NATS[NATS Client]
        
        S1 --> NATS
        S2 --> NATS
        S3 --> NATS
        
        ID1 --> NATS
        ID2 --> NATS
        ID3 --> NATS
        
        NATS --> BUS[Message Bus]
    end
    
    style CMD fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style AGG fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style PUB fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style NATS fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style BUS fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
```

Maintain correlation through event publishing:

```rust
pub trait EventPublisher: Send + Sync {
    fn publish_events(
        &self,
        events: Vec<impl DomainEvent>,
        correlation_id: CorrelationId,
    ) -> Result<(), PublishError>;
}

pub struct NatsEventPublisher {
    client: NatsClient,
    subject_mapper: SubjectMapper,
}

impl EventPublisher for NatsEventPublisher {
    fn publish_events(
        &self,
        events: Vec<impl DomainEvent>,
        correlation_id: CorrelationId,
    ) -> Result<(), PublishError> {
        for event in events {
            let subject = self.subject_mapper.event_to_subject(&event);
            let identity = MessageIdentity::new_derived_from_correlation(correlation_id.clone());
            
            self.client.publish_with_identity(
                &subject,
                &event,
                &identity
            ).await?;
        }
        Ok(())
    }
}
```

### Aggregate Repository

#### Event Sourcing Flow

```mermaid
graph TB
    subgraph "Load Aggregate"
        LOAD[Load Request<br/>ID: 123]
        SNAP{Snapshot<br/>Exists?}
        
        LOAD --> SNAP
        
        SNAP -->|Yes| LOADS[Load Snapshot<br/>Version: 50]
        SNAP -->|No| INIT[Initialize Empty]
        
        LOADS --> LOADE[Load Events<br/>After Version 50]
        INIT --> LOADALL[Load All Events]
        
        LOADE --> APPLY1[Apply Events<br/>51-75]
        LOADALL --> APPLY2[Apply Events<br/>1-75]
        
        APPLY1 --> AGG[Aggregate<br/>Version: 75]
        APPLY2 --> AGG
    end
    
    subgraph "Save Aggregate"
        SAVE[Save Request]
        UNCOMMIT[Get Uncommitted<br/>Events]
        
        SAVE --> UNCOMMIT
        UNCOMMIT --> STORE[Store Events<br/>76-80]
        
        STORE --> CHECK{Version<br/>% 10 = 0?}
        CHECK -->|Yes| SNAPSHOT[Save Snapshot<br/>Version: 80]
        CHECK -->|No| DONE[Complete]
        
        SNAPSHOT --> DONE
    end
    
    subgraph "Event Store"
        ES[Event Store]
        SS[Snapshot Store]
        
        LOADALL --> ES
        LOADE --> ES
        LOADS --> SS
        STORE --> ES
        SNAPSHOT --> SS
    end
    
    style LOAD fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style AGG fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style ES fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style SS fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
```

Integrate with event sourcing:

```rust
pub trait AggregateRepository<A: Aggregate> {
    fn load(&self, id: EntityId) -> Result<Option<A>, RepositoryError>;
    fn save(&self, aggregate: &A) -> Result<(), RepositoryError>;
}

pub struct EventSourcedRepository<A: Aggregate> {
    event_store: Arc<dyn EventStore>,
    snapshot_store: Arc<dyn SnapshotStore>,
}

impl<A: Aggregate> AggregateRepository<A> for EventSourcedRepository<A> {
    fn load(&self, id: EntityId) -> Result<Option<A>, RepositoryError> {
        // Try to load from snapshot
        let mut aggregate = if let Some(snapshot) = self.snapshot_store.load(id)? {
            A::from_snapshot(snapshot)?
        } else {
            A::new(id)
        };
        
        // Apply events since snapshot
        let events = self.event_store.load_events(id, aggregate.version())?;
        for event in events {
            aggregate.apply_event(event)?;
        }
        
        Ok(Some(aggregate))
    }
    
    fn save(&self, aggregate: &A) -> Result<(), RepositoryError> {
        let uncommitted_events = aggregate.uncommitted_events();
        
        // Save events with correlation
        for event in uncommitted_events {
            self.event_store.append(event)?;
        }
        
        // Maybe save snapshot
        if aggregate.version() % 10 == 0 {
            self.snapshot_store.save(aggregate.to_snapshot()?)?;
        }
        
        Ok(())
    }
}
```

## Domain Events

### Event Definition

Structure domain events with proper identity:

```rust
pub trait DomainEvent: Serialize + DeserializeOwned + Clone {
    fn event_type(&self) -> &str;
    fn aggregate_id(&self) -> EntityId;
    fn occurred_at(&self) -> DateTime<Utc>;
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LocationDefined {
    pub location_id: Uuid,
    pub name: String,
    pub location_type: LocationType,
    pub coordinates: Option<GeoCoordinates>,
    pub occurred_at: DateTime<Utc>,
}

impl DomainEvent for LocationDefined {
    fn event_type(&self) -> &str { "LocationDefined" }
    fn aggregate_id(&self) -> EntityId { EntityId::from_uuid(self.location_id) }
    fn occurred_at(&self) -> DateTime<Utc> { self.occurred_at }
}
```

### Event Envelope

Wrap events with routing information:

```rust
pub struct EventEnvelope<E: DomainEvent> {
    pub id: EventId,
    pub identity: MessageIdentity,
    pub event: E,
    pub metadata: EventMetadata,
}

pub struct EventMetadata {
    pub source: String,
    pub user_id: Option<String>,
    pub tenant_id: Option<String>,
    pub custom: HashMap<String, Value>,
}
```

## Process Managers and Sagas

### Process Manager Flow

```mermaid
graph TB
    subgraph "Order Fulfillment Process"
        START[Order Created Event]
        PM[Process Manager<br/>State: Started]
        
        START --> PM
        
        PM --> CMD1[Reserve Inventory<br/>Command]
        PM --> CMD2[Authorize Payment<br/>Command]
        
        subgraph "State Transitions"
            S1[Started] --> S2[Reserving Inventory]
            S2 --> S3[Processing Payment]
            S3 --> S4[Arranging Shipping]
            S4 --> S5[Complete]
        end
        
        EVT1[Inventory Reserved] --> PM
        EVT2[Payment Authorized] --> PM
        EVT3[Shipping Arranged] --> PM
        
        PM --> UPDATE[Update State]
        UPDATE --> NEXT[Generate Next Commands]
    end
    
    subgraph "Correlation Management"
        CORR[Correlation ID: 123]
        CORR -.->|Maintained Throughout| CMD1
        CORR -.-> CMD2
        CORR -.-> EVT1
        CORR -.-> EVT2
        CORR -.-> EVT3
    end
    
    style START fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style PM fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style S1 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style S5 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
    style CORR fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
```

### Process Manager

Coordinate between aggregates:

```rust
pub trait ProcessManager {
    type Command;
    type Event;
    
    fn handle_event(&mut self, event: EventEnvelope<Self::Event>) -> Vec<Self::Command>;
    fn is_complete(&self) -> bool;
}

pub struct OrderFulfillmentProcess {
    order_id: Uuid,
    state: FulfillmentState,
    correlation_id: CorrelationId,
}

impl ProcessManager for OrderFulfillmentProcess {
    type Command = FulfillmentCommand;
    type Event = FulfillmentEvent;
    
    fn handle_event(&mut self, envelope: EventEnvelope<Self::Event>) -> Vec<Self::Command> {
        let mut commands = Vec::new();
        
        match (&self.state, &envelope.event) {
            (FulfillmentState::Started, FulfillmentEvent::OrderCreated { items, .. }) => {
                // Reserve inventory for each item
                for item in items {
                    commands.push(FulfillmentCommand::ReserveInventory {
                        item_id: item.id,
                        quantity: item.quantity,
                    });
                }
                self.state = FulfillmentState::ReservingInventory;
            }
            (FulfillmentState::ReservingInventory, FulfillmentEvent::InventoryReserved { .. }) => {
                // Check if all items reserved
                if self.all_items_reserved() {
                    commands.push(FulfillmentCommand::ProcessPayment {
                        order_id: self.order_id,
                    });
                    self.state = FulfillmentState::ProcessingPayment;
                }
            }
            // ... more state transitions
        }
        
        // Maintain correlation for all commands
        commands.into_iter()
            .map(|cmd| self.wrap_with_correlation(cmd))
            .collect()
    }
}
```

### Saga Implementation

#### Saga Pattern Visualization

```mermaid
graph TB
    subgraph "Saga Execution"
        START[Start Saga]
        
        subgraph "Forward Path"
            T1[Transaction 1<br/>Reserve Inventory]
            T2[Transaction 2<br/>Charge Payment]
            T3[Transaction 3<br/>Book Shipping]
            SUCCESS[All Complete]
            
            START --> T1
            T1 -->|Success| T2
            T2 -->|Success| T3
            T3 -->|Success| SUCCESS
        end
        
        subgraph "Compensation Path"
            FAIL[Failure at T3]
            C3[Compensate T3<br/>Cancel Shipping]
            C2[Compensate T2<br/>Refund Payment]
            C1[Compensate T1<br/>Release Inventory]
            ROLLBACK[Rollback Complete]
            
            T3 -->|Failure| FAIL
            FAIL --> C3
            C3 --> C2
            C2 --> C1
            C1 --> ROLLBACK
        end
    end
    
    subgraph "Saga State"
        STATE[Saga State<br/>Steps: [T1, T2, T3]<br/>Current: 2<br/>Status: Compensating]
        LOG[Event Log<br/>T1: Success<br/>T2: Success<br/>T3: Failed]
    end
    
    style START fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style SUCCESS fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style FAIL fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
    style T1 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style T2 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style T3 fill:#4ECDC4,stroke:#2B8A89,stroke-width:2px,color:#FFF
    style C3 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style C2 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style C1 fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
```

Implement distributed transactions with compensations:

```rust
pub trait Saga {
    type Command;
    type Event;
    
    fn start(&mut self) -> Vec<Self::Command>;
    fn handle_event(&mut self, event: Self::Event) -> SagaAction<Self::Command>;
    fn compensate(&mut self) -> Vec<Self::Command>;
}

pub enum SagaAction<C> {
    Continue(Vec<C>),
    Compensate(Vec<C>),
    Complete,
}

pub struct OrderSaga {
    order_id: Uuid,
    steps: Vec<SagaStep>,
    current_step: usize,
    correlation_id: CorrelationId,
}

impl Saga for OrderSaga {
    type Command = OrderCommand;
    type Event = OrderEvent;
    
    fn handle_event(&mut self, event: Self::Event) -> SagaAction<Self::Command> {
        match event {
            OrderEvent::PaymentFailed { .. } => {
                // Start compensation
                SagaAction::Compensate(self.compensate())
            }
            OrderEvent::StepCompleted { step_id } => {
                self.current_step += 1;
                if self.current_step < self.steps.len() {
                    SagaAction::Continue(vec![self.steps[self.current_step].command.clone()])
                } else {
                    SagaAction::Complete
                }
            }
            _ => SagaAction::Continue(vec![])
        }
    }
    
    fn compensate(&mut self) -> Vec<Self::Command> {
        self.steps[..self.current_step]
            .iter()
            .rev()
            .filter_map(|step| step.compensation.clone())
            .collect()
    }
}
```

## Query Handling

### CQRS Architecture

```mermaid
graph TB
    subgraph "Write Side"
        CMD[Commands]
        AGG[Aggregates]
        EVT[Domain Events]
        ES[Event Store]
        
        CMD --> AGG
        AGG --> EVT
        EVT --> ES
    end
    
    subgraph "Read Side"
        PROJ[Projections]
        RM1[Location Read Model]
        RM2[Search Index]
        RM3[Analytics View]
        
        ES --> PROJ
        PROJ --> RM1
        PROJ --> RM2
        PROJ --> RM3
    end
    
    subgraph "Query Handling"
        QRY[Queries]
        QH[Query Handlers]
        
        QRY --> QH
        QH --> RM1
        QH --> RM2
        QH --> RM3
        
        QH --> RESP[Query Response]
    end
    
    subgraph "Eventual Consistency"
        EVT -.->|Async| PROJ
        NOTE[Read models may lag<br/>behind write model]
    end
    
    style CMD fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style AGG fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style ES fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style PROJ fill:#95E1D3,stroke:#63C7B8,stroke-width:3px,color:#000
    style QH fill:#2D3436,stroke:#000,stroke-width:3px,color:#FFF
```

### CQRS Read Models

Separate read models with proper querying:

```rust
pub trait QueryHandler<Q: Query> {
    type Result;
    
    fn handle(&self, query: Q) -> Result<Self::Result, QueryError>;
}

pub struct LocationQueryHandler {
    read_store: Arc<dyn ReadModelStore>,
}

impl QueryHandler<FindLocationByCoordinates> for LocationQueryHandler {
    type Result = Vec<LocationReadModel>;
    
    fn handle(&self, query: FindLocationByCoordinates) -> Result<Self::Result, QueryError> {
        self.read_store.find_by_coordinates(
            query.center,
            query.radius_meters
        )
    }
}
```

### Projection Management

#### Projection Update Flow

```mermaid
sequenceDiagram
    participant ES as Event Store
    participant PS as Projection Service
    participant P as Projection
    participant RM as Read Model Store
    participant C as Cache
    
    ES->>PS: Event Stream
    PS->>PS: Filter Relevant Events
    
    loop For Each Event
        PS->>P: Handle Event
        P->>P: Transform to Read Model
        P->>RM: Update/Insert
        RM->>C: Invalidate Cache
        P->>PS: Acknowledge
    end
    
    PS->>PS: Update Checkpoint
    
    note over PS: Track last processed<br/>event for restart
```

Update read models from events:

```rust
pub trait Projection {
    type Event;
    
    fn handle_event(&mut self, event: EventEnvelope<Self::Event>) -> Result<(), ProjectionError>;
}

pub struct LocationProjection {
    read_store: Arc<dyn ReadModelStore>,
}

impl Projection for LocationProjection {
    type Event = LocationEvent;
    
    fn handle_event(&mut self, envelope: EventEnvelope<Self::Event>) -> Result<(), ProjectionError> {
        match envelope.event {
            LocationEvent::LocationDefined { location_id, name, coordinates, .. } => {
                let read_model = LocationReadModel {
                    id: location_id,
                    name,
                    coordinates,
                    last_updated: envelope.event.occurred_at(),
                };
                self.read_store.upsert(read_model)?;
            }
            LocationEvent::LocationMoved { location_id, new_coordinates, .. } => {
                self.read_store.update_coordinates(location_id, new_coordinates)?;
            }
            // ... handle other events
        }
        Ok(())
    }
}
```

## Integration Patterns

### Domain Service Communication

```mermaid
graph TB
    subgraph "Order Service"
        OS[Order Service]
        OC[Order Commands]
        OE[Order Events]
    end
    
    subgraph "Inventory Service"
        IS[Inventory Service]
        IC[Inventory Commands]
        IE[Inventory Events]
        IQ[Inventory Queries]
    end
    
    subgraph "Communication Flow"
        OS -->|Check Stock| REQ[inventory.queries.availability.check]
        REQ --> IQ
        IQ --> RESP[inventory.responses.availability.result]
        RESP --> OS
        
        OE -->|Order Created| ASYNC[order.events.order.created]
        ASYNC -.->|Subscribe| IS
        IS -->|Reserve Stock| IC
    end
    
    subgraph "Message Identity"
        ID1[Request ID: 123<br/>Correlation: ABC]
        ID2[Response ID: 456<br/>Correlation: ABC]
        ID3[Event ID: 789<br/>Correlation: ABC]
        
        REQ --> ID1
        RESP --> ID2
        ASYNC --> ID3
    end
    
    style OS fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style IS fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style REQ fill:#FFE66D,stroke:#FCC419,stroke-width:2px,color:#000
    style RESP fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

### Domain Service Integration

```rust
pub trait DomainService {
    fn subject_prefix(&self) -> &str;
}

pub struct InventoryService {
    nats_client: NatsClient,
}

impl DomainService for InventoryService {
    fn subject_prefix(&self) -> &str {
        "inventory"
    }
}

impl InventoryService {
    pub async fn check_availability(&self, item_id: Uuid) -> Result<bool, ServiceError> {
        let query_subject = format!("{}.queries.availability.check", self.subject_prefix());
        let response_subject = format!("{}.responses.availability.result", self.subject_prefix());
        
        let query = CheckAvailability { item_id };
        let identity = MessageIdentity::new_root();
        
        let response = self.nats_client
            .request_with_identity(&query_subject, &query, &identity)
            .await?;
        
        Ok(response.available)
    }
}
```

### Event Notification

#### Event Notification Flow

```mermaid
graph TB
    subgraph "Internal Domain Event"
        EVT[LocationDefined Event]
        META[Event Metadata<br/>- Aggregate ID<br/>- User ID<br/>- Timestamp]
    end
    
    subgraph "Notification Service"
        NS[Notification Service]
        MAPPER[Event Mapper]
        FILTER{Should<br/>Notify?}
        
        EVT --> NS
        NS --> FILTER
        FILTER -->|Yes| MAPPER
        FILTER -->|No| SKIP[Skip]
    end
    
    subgraph "Public Notifications"
        PUB1[public.notifications.location.created]
        PUB2[public.notifications.location.updated]
        WEBHOOK[Webhook Dispatcher]
        EMAIL[Email Service]
        PUSH[Push Notifications]
        
        MAPPER --> PUB1
        MAPPER --> PUB2
        
        PUB1 --> WEBHOOK
        PUB1 --> EMAIL
        PUB1 --> PUSH
    end
    
    subgraph "External Systems"
        EXT1[Partner API]
        EXT2[Analytics]
        EXT3[Mobile App]
        
        WEBHOOK --> EXT1
        EMAIL --> EXT2
        PUSH --> EXT3
    end
    
    style EVT fill:#FF6B6B,stroke:#C92A2A,stroke-width:3px,color:#FFF
    style NS fill:#4ECDC4,stroke:#2B8A89,stroke-width:3px,color:#FFF
    style MAPPER fill:#FFE66D,stroke:#FCC419,stroke-width:3px,color:#000
    style PUB1 fill:#95E1D3,stroke:#63C7B8,stroke-width:2px,color:#000
```

Publish domain notifications:

```rust
pub struct DomainNotificationService {
    publisher: Arc<dyn EventPublisher>,
    notification_mapper: NotificationMapper,
}

impl DomainNotificationService {
    pub async fn notify_domain_event<E: DomainEvent>(
        &self,
        event: E,
        correlation_id: CorrelationId,
    ) -> Result<(), NotificationError> {
        // Map to public notification
        let notification = self.notification_mapper.map_event(event)?;
        
        // Publish with proper subject
        let subject = format!("public.notifications.{}", notification.notification_type());
        
        self.publisher.publish_with_subject(
            &subject,
            notification,
            correlation_id
        ).await?;
        
        Ok(())
    }
}
```

## Best Practices

### 1. Maintain Bounded Context Isolation

```rust
// Good: Clear context boundaries
mod location_context {
    pub const SUBJECT_PREFIX: &str = "location";
    
    pub fn command_subject(aggregate: &str, command: &str) -> String {
        format!("{}.commands.{}.{}", SUBJECT_PREFIX, aggregate, command)
    }
}

// Bad: Crossing context boundaries directly
fn bad_example() {
    // Don't directly call other contexts
    let subject = "inventory.commands.stock.update"; // Wrong context!
}
```

### 2. Use Anti-Corruption Layers

```rust
// Good: Translate at boundaries
pub struct OrderToInventoryTranslator {
    pub fn translate_order_item(&self, order_item: OrderItem) -> InventoryRequest {
        InventoryRequest {
            sku: order_item.product_sku,
            quantity: order_item.quantity,
            warehouse_id: self.determine_warehouse(order_item.shipping_address),
        }
    }
}
```

### 3. Leverage Process Managers for Coordination

```rust
// Good: Process manager coordinates between aggregates
pub struct CheckoutProcessManager {
    pub fn coordinate_checkout(&mut self, order_id: Uuid) -> Vec<Command> {
        vec![
            Command::ValidateInventory { order_id },
            Command::AuthorizePayment { order_id },
            Command::AllocateShipping { order_id },
        ]
    }
}
```

### 4. Keep Aggregates Small

```rust
// Good: Focused aggregate
pub struct Location {
    id: LocationId,
    name: String,
    coordinates: GeoCoordinates,
    location_type: LocationType,
}

// Bad: Aggregate doing too much
pub struct BadLocation {
    id: LocationId,
    name: String,
    coordinates: GeoCoordinates,
    weather_data: WeatherData,        // Should be separate aggregate
    traffic_data: TrafficData,        // Should be separate aggregate
    business_listings: Vec<Business>, // Should be separate aggregate
}
```