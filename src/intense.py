import urllib3
import json

http = urllib3.PoolManager()

def solve (puzzle):
    req = json.dumps({'puzzle' : puzzle})
    resp = http.request(
        "POST", "http://localhost:7878/api/solve", 
        body=req,
        headers={'Content-Type': 'application/json'})
    solution = json.loads(resp.data.decode("utf-8"))
    if solution['status'] == "success":
        return True
    else:
        return False

if __name__ == "__main__":
    from multiprocessing import Pool
    import os, sys, getopt, time
    
    nb_req = 10000
    help = 'intense.py -n <number of requests>'
    try:
        opts, _ = getopt.getopt(sys.argv[1::],"hn:",["help","nbreq="])
    except getopt.GetoptError:
        print(help)
        sys.exit(2)
    for opt, arg in opts:
        if opt in ('-h', "--help"):
            print(help)
            sys.exit()
        elif opt in ("-n", "--nbreq"):
            try:
                nb_req = int(arg)
            except:
                print(help)
                sys.exit(2) 

    ts = time.time()

    with Pool(os.cpu_count()) as p:
        solved = p.map(solve, ("700000600060001070804020005000470000089000340000039000600050709010300020003000004" for _ in range(nb_req)))

    print('{}/{} puzzles solved'.format(solved.count(True), len(solved)))
    print("{:.5f} sec.".format(round(time.time() - ts, 5)))
