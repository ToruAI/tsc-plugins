## Frontend Specification

### Requirement: Tab Navigation
The frontend SHALL provide tabbed navigation between views.

#### Scenario: Three tabs available
- **WHEN** the plugin loads
- **THEN** three tabs are visible: "Timers", "History", and "Settings"
- **AND** Timers tab is selected by default
- **AND** each tab has an icon (Clock, History, Settings)

### Requirement: Timer Card Display
The frontend SHALL display each timer as a card with schedule and status.

#### Scenario: Status indicator bar
- **WHEN** displaying a timer card
- **THEN** show colored left border indicating status
- **AND** green for success, red for failed, amber for running

#### Scenario: Schedule display
- **WHEN** displaying a timer card
- **THEN** show human-readable schedule (e.g., "Mon-Fri 08-21:00")

#### Scenario: Timing display
- **WHEN** displaying a timer card
- **THEN** show next run as relative time (e.g., "in 45m")
- **AND** show last run as relative time (e.g., "15m ago")

#### Scenario: Disabled badge
- **WHEN** timer is disabled
- **THEN** show "OFF" badge next to timer name

#### Scenario: Action buttons with loading states
- **WHEN** displaying a timer card
- **THEN** show "Run" button (Play icon) for full production run
- **AND** show "Test" button (TestTube icon) for test run
- **AND** show "Toggle" button (Power icon) colored by state
- **WHEN** an action is in progress
- **THEN** show spinner on the active button
- **AND** disable all buttons until action completes

### Requirement: Toast Notifications
The frontend SHALL show toast notifications for user feedback.

#### Scenario: Action success
- **WHEN** user triggers run/test/toggle action
- **THEN** show success toast with timer name
- **AND** toast appears at top-center

#### Scenario: Action failure
- **WHEN** an action fails
- **THEN** show error toast with error message
- **AND** toast is styled in red (destructive)

### Requirement: History Tab
The frontend SHALL display execution history with filtering and detail view.

#### Scenario: Default view shows all
- **WHEN** History tab is active
- **THEN** show combined history from all timers by default
- **AND** display timer name in each history row

#### Scenario: Timer filter
- **WHEN** History tab is active
- **THEN** show dropdown with "All timers" and individual timers
- **AND** default selection is "All timers"
- **WHEN** specific timer is selected
- **THEN** filter history to show only that timer
- **AND** hide timer name column (redundant)

#### Scenario: History list
- **WHEN** viewing history
- **THEN** display cards with: Status icon, Time, Duration, Trigger icon
- **AND** status uses shared StatusIcon component
- **AND** manual triggers show lightning bolt (blue)
- **AND** scheduled triggers show calendar icon

#### Scenario: Execution detail dialog
- **WHEN** user clicks a history row
- **THEN** open modal with timer name in title
- **AND** show: Started time, Duration, Exit Code, Trigger
- **AND** show scrollable output log with timestamps
- **WHEN** loading details
- **THEN** show spinner in dialog

### Requirement: Settings Tab
The frontend SHALL allow selecting which timers to watch.

#### Scenario: Timer selection
- **WHEN** Settings tab is active
- **THEN** show searchable list of all available timers
- **AND** selected timers have highlighted background
- **AND** show checkbox indicator for each timer

#### Scenario: Search filter
- **WHEN** user types in search
- **THEN** filter timers by name or description

#### Scenario: Save settings
- **WHEN** user clicks Save
- **THEN** persist to backend
- **AND** show success toast

### Requirement: Status Colors and Icons
The frontend SHALL use consistent color coding via shared components.

#### Scenario: Shared StatusIcon component
- **GIVEN** status is "success"
- **THEN** render CheckCircle icon in emerald-500
- **GIVEN** status is "failed"
- **THEN** render XCircle icon in red-500
- **GIVEN** status is "running"
- **THEN** render Clock icon in amber-500 with pulse animation

#### Scenario: Status bar colors
- **GIVEN** a status value
- **THEN** getStatusBarColor() returns appropriate Tailwind class

### Requirement: Shared Utilities
The frontend SHALL use shared formatting utilities.

#### Scenario: formatRelativeTime
- **GIVEN** a timestamp
- **THEN** return relative time string (e.g., "45m", "2h", "3d", "<1m")

#### Scenario: formatDuration
- **GIVEN** duration in seconds
- **THEN** return human-readable string (e.g., "45s", "2m 30s")

#### Scenario: formatTime
- **GIVEN** a timestamp
- **THEN** return time if today (e.g., "14:30")
- **OR** return date+time if other day (e.g., "Jan 15, 14:30")

#### Scenario: displayTimerName
- **GIVEN** a timer name with ".timer" suffix
- **THEN** return name without suffix

### Requirement: Auto Refresh
The frontend SHALL periodically refresh timer status.

#### Scenario: Automatic updates
- **WHEN** Timers tab is active
- **THEN** refresh data every 60 seconds
- **AND** show loading indicator during refresh

### Requirement: Development Mode
The frontend SHALL support development without backend.

#### Scenario: MSW mock service
- **WHEN** running in dev mode (npm run dev)
- **THEN** MSW intercepts API requests
- **AND** returns realistic mock data
- **AND** mock state persists during session (run/toggle updates)

#### Scenario: Production build exclusion
- **WHEN** building for production
- **THEN** mock code is NOT included in bundle
- **AND** bundle size is not affected by mocks

### Requirement: Data Types

#### ExecutionHistory type
```typescript
interface ExecutionHistory {
  invocation_id: string;
  timer_name: string;      // Added: identifies timer in combined view
  start_time: string;
  end_time: string | null;
  duration_secs: number | null;
  status: "success" | "failed" | "running";
  exit_code: number | null;
  trigger: "scheduled" | "manual";
}
```
