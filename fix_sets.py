import re

with open('src/lib/components/dashboard/FilterBar.svelte', 'r') as f:
    content = f.read()

content = content.replace("new Set<string>()", "new SvelteSet<string>()")

if "SvelteSet" not in content[:500]:
    content = content.replace(
        "import { t } from 'svelte-i18n';",
        "import { t } from 'svelte-i18n';\n\timport { SvelteSet } from 'svelte/reactivity';"
    )

with open('src/lib/components/dashboard/FilterBar.svelte', 'w') as f:
    f.write(content)

