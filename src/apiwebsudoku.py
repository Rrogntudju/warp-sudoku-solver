from bs4 import BeautifulSoup
import urllib3, time, json

http = urllib3.PoolManager()

# Extract the evil puzzle link from the frame
frameset_page = http.request("get", "http://www.websudoku.com/?level=4")
soup = BeautifulSoup(frameset_page.data, "html.parser")
puzzle_link = soup("frame")[0]["src"] 

# Extract the puzzle's values and add the empties
puzzle_page = http.request("get", puzzle_link)
soup = BeautifulSoup(puzzle_page.data, "html.parser")
puzzle_values = {tag["id"] : tag["value"] for tag in soup("input", readonly=True)}
ids = ("f" + str(j) + str(i) for i in range(9) for j in range(9))
puzzle = "".join(puzzle_values[id] if id in puzzle_values else "0" for id in ids)

# Solve with sudoku web api
api_url = "http://localhost:7878/api"
req = json.dumps({'puzzle' : puzzle})
resp = http.request(
    "POST", api_url + "/display", 
    body=req,
    headers={'Content-Type': 'application/json'})
display_grid = json.loads(resp.data.decode("utf-8"))
print()
if display_grid['status'] == "success":
    for line in display_grid['data']:
        print(line)
else:
    print(display_grid['message'])
print()

ts = time.time()
resp = http.request(
    "POST", api_url + "/solve", 
    body=req,
    headers={'Content-Type': 'application/json'})
ts = time.time() - ts

solution = json.loads(resp.data.decode("utf-8"))
if solution['status'] == "success":
    resp = http.request(
        "POST", api_url + "/display", 
        body=json.dumps({'puzzle' : solution['data']}),
        headers={'Content-Type': 'application/json'})
    display_grid = json.loads(resp.data.decode("utf-8"))
    if display_grid['status'] == "success":
        for line in display_grid['data']:
            print(line)
    else:
        print(display_grid['message'])

    print("\n{:.5f} sec.\n".format(round(ts, 5)))
else:
    print(solution['message'])
