with open('src/lib/components/dashboard/IssueTable.svelte', 'r') as f:
    content = f.read()

bad_each = """			{#each issues as issue (issue.id)}
				<tr"""

good_each = """			{#each issues as issue (issue.id)}
				{@const priority = resolvePriority(issue)}
				{@const severity = resolveSeverity(issue)}
				{@const type = resolveType(issue)}
				<tr"""

content = content.replace(bad_each, good_each)

# remove the inner consts
content = content.replace("""					<td class="px-2 py-2.5">
						{@const priority = resolvePriority(issue)}
						{@const severity = resolveSeverity(issue)}
						<div class="flex flex-col gap-1">""", """					<td class="px-2 py-2.5">
						<div class="flex flex-col gap-1">""")

content = content.replace("""					<td class="px-2 py-2.5">
						{@const type = resolveType(issue)}
						{#if type}""", """					<td class="px-2 py-2.5">
						{#if type}""")

with open('src/lib/components/dashboard/IssueTable.svelte', 'w') as f:
    f.write(content)
print("Patcher executed successfully.")
