# Raise import error if cli subpackage is imported directly.
raise ImportError(
    'CLI shubpackage should not be imported. Use from the command line instead.'
)
