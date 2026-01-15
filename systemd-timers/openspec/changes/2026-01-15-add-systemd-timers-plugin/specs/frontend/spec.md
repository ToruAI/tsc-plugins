## ADDED Requirements

### Requirement: Tab Navigation
The frontend SHALL provide tabbed navigation between views.

#### Scenario: Three tabs available
- **WHEN** the plugin loads
- **THEN** three tabs are visible: "Timers", "History", and "Settings"
- **AND** Timers tab is selected by default

### Requirement: Timer Card Display
The frontend SHALL display each timer as a card with schedule and status.

#### Scenario: Schedule display
- **WHEN** displaying a timer card
- **THEN** show human-readable schedule (e.g., "Mon-Fri 08-21:00")

#### Scenario: Next run display
- **WHEN** displaying a timer card
- **THEN** show next run as relative time (e.g., "in 45 min")

#### Scenario: Last run status
- **WHEN** last run succeeded
- **THEN** display green checkmark (✅) with relative time
- **WHEN** last run failed
- **THEN** display red X (❌) with relative time

#### Scenario: Action buttons
- **WHEN** displaying a timer card
- **THEN** show "Run" button for full production run
- **AND** show "Test" button for test run
- **AND** show "Enable" or "Disable" button based on current state

### Requirement: History Tab
The frontend SHALL display execution history with filtering and detail view.

#### Scenario: Task selection
- **WHEN** History tab is active
- **THEN** show dropdown to select which timer's history to view

#### Scenario: History table
- **WHEN** a timer is selected
- **THEN** display table with columns: Time, Status, Duration, Trigger
- **AND** status is color-coded (green=success, red=failed, yellow=running)

#### Scenario: Pagination
- **WHEN** history has more than one page
- **THEN** show pagination controls

#### Scenario: Execution detail dialog
- **WHEN** user clicks a history row
- **THEN** open modal with full execution details
- **AND** show scrollable output log with timestamps

### Requirement: Settings Tab
The frontend SHALL allow selecting which timers to watch.

#### Scenario: Timer selection
- **WHEN** Settings tab is active
- **THEN** show list of all available timers with checkboxes
- **AND** previously selected timers are checked

#### Scenario: Save settings
- **WHEN** user changes selection
- **THEN** save to KV storage
- **AND** show success toast

### Requirement: Status Colors
The frontend SHALL use consistent color coding.

#### Scenario: Success status
- **WHEN** execution succeeded (exit code 0)
- **THEN** use green color (#22c55e) and ✅ icon

#### Scenario: Failed status
- **WHEN** execution failed (exit code != 0)
- **THEN** use red color (#ef4444) and ❌ icon

#### Scenario: Running status
- **WHEN** execution is in progress
- **THEN** use yellow color (#eab308) and ⏳ icon

### Requirement: Auto Refresh
The frontend SHALL periodically refresh timer status.

#### Scenario: Automatic updates
- **WHEN** Timers tab is active
- **THEN** refresh data every 60 seconds
