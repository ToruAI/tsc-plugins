## ADDED Requirements

### Requirement: Plugin Metadata
The plugin SHALL identify itself to TSC with standard metadata.

#### Scenario: Metadata output
- **WHEN** the binary is run with --metadata flag
- **THEN** it outputs JSON with id, name, version, icon, and route fields
- **AND** id is "systemd-services"
- **AND** route is "/systemd-services"

### Requirement: Service Listing API
The plugin SHALL provide endpoints to list systemd services.

#### Scenario: List watched services
- **WHEN** a client requests GET /services
- **THEN** the plugin returns only services stored in KV watched_services
- **AND** each service includes name, status, active_state, sub_state, uptime_seconds

#### Scenario: List all available services
- **WHEN** a client requests GET /services/available
- **THEN** the plugin returns all systemd services on the system
- **AND** each service includes name and description

### Requirement: Service Control API
The plugin SHALL provide endpoints to control systemd services.

#### Scenario: Start service
- **WHEN** a client sends POST /services/:name/start
- **THEN** the plugin executes systemctl start :name
- **AND** returns success or error message

#### Scenario: Stop service
- **WHEN** a client sends POST /services/:name/stop
- **THEN** the plugin executes systemctl stop :name
- **AND** returns success or error message

#### Scenario: Restart service
- **WHEN** a client sends POST /services/:name/restart
- **THEN** the plugin executes systemctl restart :name
- **AND** returns success or error message

### Requirement: Log Retrieval API
The plugin SHALL provide endpoint to retrieve service logs.

#### Scenario: Get recent logs
- **WHEN** a client requests GET /services/:name/logs?lines=100
- **THEN** the plugin executes journalctl -u :name -n 100
- **AND** returns array of log entries with timestamp and message

### Requirement: KV Storage
The plugin SHALL persist settings using TSC's KV storage.

#### Scenario: Store watched services
- **WHEN** settings are saved
- **THEN** watched_services key contains JSON array of service names

#### Scenario: Load watched services
- **WHEN** plugin initializes or services are requested
- **THEN** it reads watched_services from KV storage
