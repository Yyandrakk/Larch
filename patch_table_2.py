with open('src/lib/components/dashboard/IssueTable.svelte', 'r') as f:
    content = f.read()

bad_div = """					<td class="px-2 py-2.5">
						<div class="flex flex-col gap-1">
							{@const priority = resolvePriority(issue)}
							{@const severity = resolveSeverity(issue)}"""

good_div = """					<td class="px-2 py-2.5">
						{@const priority = resolvePriority(issue)}
						{@const severity = resolveSeverity(issue)}
						<div class="flex flex-col gap-1">"""

content = content.replace(bad_div, good_div)

with open('src/lib/components/dashboard/IssueTable.svelte', 'w') as f:
    f.write(content)
print("Patcher executed successfully.")
