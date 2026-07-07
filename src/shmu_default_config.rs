pub const DEFAULT_CONFIG: &str = r#"
# List of areas to monitor for alerts.
# Use the area name as provided by the alert provider.
# An empty list means no areas are configured and configuration is invalid.
# Check https://www.shmu.sk/sk/?page=1680 for list of areas
area = []

# Logging verbosity level.
# The selected level includes all higher-severity messages:
#   info  = info + warn + error
#   warn  = warn + error
#   error = error only
# values: "error", "warn", "info"
log_level = "warn"

# How often alerts should be fetched, in minutes.
# Lower values provide faster updates but create more requests.
period = 60

# Whether to include the full alert description in notification messages.
include_description = false

# Whether desktop notifications should be displayed.
# Disable this for headless servers or silent operation.
notifications = true

# Minimum alert severity to process.
# Available values:
# - "mild"
# - "moderate"
# - "severe"
# - "extreme"
#
# Alerts below this severity will be ignored.
min_severity = "mild"

# Alert type filtering.
#
# If this list is empty, all alert types are processed
# regardless of the value of alert_types_is_allowlist.
#
# Available alert type IDs:
#  1  - Wind
#  2  - Snow and Ice
#  3  - Thunderstorm
#  4  - Fog
#  5  - High Temperature
#  6  - Low Temperature
#  7  - Coastal Event
#  8  - Forest Fire
#  9  - Avalanches
# 10  - Rain
# 11  - Unknown (legacy value)
# 12  - Flooding
# 13  - Rain Flood
# 14  - Marine Hazard
# 15  - Drought
#
# Example:
# alert_types = [1, 2, 3]
alert_types = []

# Controls how alert_types is interpreted.
#
# false:
#   The listed alert types are excluded from processing.
#
# true:
#   Only the listed alert types are processed.
#
# This option has no effect when alert_types is empty.
alert_types_is_allowlist = false
"#;