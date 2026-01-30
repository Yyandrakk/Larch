from playwright.sync_api import sync_playwright

def verify(page):
    print("Navigating to localhost:1420")
    page.goto("http://localhost:1420")
    print("Waiting for networkidle")
    # Wait for some content to appear
    try:
        page.wait_for_load_state("networkidle", timeout=5000)
    except:
        print("Timeout waiting for networkidle, proceeding...")

    print("Taking screenshot")
    page.screenshot(path="login_page.png")

if __name__ == "__main__":
    with sync_playwright() as p:
        browser = p.chromium.launch(headless=True)
        page = browser.new_page()
        verify(page)
        browser.close()
