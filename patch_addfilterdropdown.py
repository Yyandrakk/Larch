import re

with open('src/lib/components/dashboard/filters/AddFilterDropdown.svelte', 'r') as f:
    content = f.read()

# Update props destructuring
content = content.replace(
    "\t\thasAssigneeFilter = false\n\t}: {",
    "\t\thasAssigneeFilter = false,\n\t\thasPriorityFilter = false,\n\t\thasSeverityFilter = false,\n\t\thasTypeFilter = false,\n\t\tonSelectPriority,\n\t\tonSelectSeverity,\n\t\tonSelectType\n\t}: {"
)

# Update types
content = content.replace(
    "\t\thasAssigneeFilter?: boolean;\n\t} = $props();",
    "\t\thasAssigneeFilter?: boolean;\n\t\thasPriorityFilter?: boolean;\n\t\thasSeverityFilter?: boolean;\n\t\thasTypeFilter?: boolean;\n\t\tonSelectPriority?: () => void;\n\t\tonSelectSeverity?: () => void;\n\t\tonSelectType?: () => void;\n\t} = $props();"
)

with open('src/lib/components/dashboard/filters/AddFilterDropdown.svelte', 'w') as f:
    f.write(content)

print("AddFilterDropdown props patched")
