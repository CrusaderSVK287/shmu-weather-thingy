# SHMÚ Storm Watcher - Development Roadmap

## Phase 1 – Basic Prototype

* ✅ Poll SHMÚ for new CAP alerts every minute.
* ✅ Download the latest CAP XML file(s). 
* ✅ Parse the XML data. 
* ✅ If an alert affects the configured district(s), display it in the terminal. 

---

## Phase 2 – Avoid Duplicate Processing

* ✅ Detect whether the fetched data is new.
* ✅ Skip parsing if no new data is available.
* ❔ Store processed alert IDs (or timestamps) to avoid handling the same alert multiple times. (Example: <identifier>2.49.0.0.703.0.SK.260706081300.6a4b63a9_RK</identifier>) sqlite

---

## Phase 3 – Desktop Notifications

* ✅ Add native desktop notifications for Windows and Linux.

---

## Phase 4 – Configuration

* ✅ Move hardcoded values into a configuration file.
* ✅ Make the following configurable:

  * ✅ Poll interval
  * ✅ District(s) to monitor
  * ✅ Warning/event types
  * ✅ Minimum warning severity
  * How many hours before the start of an event a notification should be sent (for example allert is issued in sunday but event starts wednesday)

* ✅Configuration file paths

---

## Phase 5 – Remote Notifications

* Add additional notification backends, such as:

  * ntfy
  * Telegram
  * Discord
  * Email
* Allow selecting one or more notification methods via configuration.

# Needed Polish
- remove blocking operations from async context. tokio expects avoid blocking the runtime thread
- mixing async and blocking paradigms, look up timer-based scheduling or a stream-like
- explore Option<T>, check last_fetched_url: String
- Configuration: Fix panicking when invalid config. Check constructor. Also fix the case sensitivity for config... while not really a bug, its annoying
- custom icons for the notifications would be nice depending on the alert type

# Icons
- api to handle icons.
- ✅Individual icons:
  - ✅Wind
  - ✅Snow and Ice
  - ✅Thunderstorm
  - ✅Fog
  - ✅High Temperature
  - ✅Low Temperature
  - ✅Coastal Event
  - ✅Forest Fire
  - ✅Avalanches
  - ✅Rain
  - ✅Unknown (legacy value)
  - ✅Flooding
  - ✅Rain Flood
  - ✅Marine Hazard
  - ✅Drought
