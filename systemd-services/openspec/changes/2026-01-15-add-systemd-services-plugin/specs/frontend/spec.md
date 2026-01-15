## ADDED Requirements

### Requirement: Tab Navigation
The frontend SHALL provide tabbed navigation between views.

#### Scenario: Two tabs available
- **WHEN** the plugin loads
- **THEN** two tabs are visible: "Services" and "Settings"
- **AND** Services tab is selected by default

### Requirement: Service Card Display
The frontend SHALL display each service as a card with status and actions.

#### Scenario: Status indicator
- **WHEN** a service is running
- **THEN** display green indicator (🟢)
- **WHEN** a service is failed
- **THEN** display red indicator (🔴)
- **WHEN** a service is inactive
- **THEN** display gray indicator (⚪)

#### Scenario: Service information
- **WHEN** displaying a service card
- **THEN** show service name, status text, and uptime

#### Scenario: Action buttons
- **WHEN** service is running
- **THEN** show Restart, Stop, and Logs buttons
- **WHEN** service is stopped
- **THEN** show Start and Logs buttons

### Requirement: Log Dialog
The frontend SHALL display logs in a modal dialog.

#### Scenario: Open logs
- **WHEN** user clicks Logs button
- **THEN** modal opens with service name as title
- **AND** log content is fetched and displayed

#### Scenario: Log display
- **WHEN** logs are displayed
- **THEN** use monospace font
- **AND** show scrollable content

### Requirement: Settings Tab
The frontend SHALL allow selecting which services to watch.

#### Scenario: Service selection
- **WHEN** Settings tab is active
- **THEN** show list of all available services with checkboxes
- **AND** previously selected services are checked

#### Scenario: Save settings
- **WHEN** user changes selection
- **THEN** save to KV storage
- **AND** show success toast

### Requirement: Auto Refresh
The frontend SHALL periodically refresh service status.

#### Scenario: Automatic updates
- **WHEN** Services tab is active
- **THEN** refresh data every 30 seconds
