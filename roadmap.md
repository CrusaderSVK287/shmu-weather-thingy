# SHMÚ Storm Watcher - Development Roadmap

## Phase 1 – Basic Prototype

* ✅ Poll SHMÚ for new CAP alerts every minute.
* ✅ Download the latest CAP XML file(s). 
* ✅ Parse the XML data. 
* ✅ If an alert affects the configured district(s), display it in the terminal. 

---

## Phase 2 – Avoid Duplicate Processing

* Detect whether the fetched data is new.
* Skip parsing if no new data is available.
* Store processed alert IDs (or timestamps) to avoid handling the same alert multiple times.

---

## Phase 3 – Desktop Notifications

* Add native desktop notifications for Windows and Linux.
* Keep terminal output for debugging and logging.

---

## Phase 4 – Configuration

* Move hardcoded values into a configuration file.
* Make the following configurable:

  * Poll interval
  * District(s) to monitor
  * Warning/event types
  * Minimum warning severity

---

## Phase 5 – Remote Notifications

* Add additional notification backends, such as:

  * ntfy
  * Telegram
  * Discord
  * Email
* Allow selecting one or more notification methods via configuration.
