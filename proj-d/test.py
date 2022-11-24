import requests
import time
url = "http://localhost:23000/D"
data = {"class1":"1 2 3 4","class2":"5 6 7 8","class3":"9 10 11 12","rand_type":"1"}
headers = {
        "Content-Type": "application/x-www-form-urlencoded; charset=UTF-8",
        'User-Agent': "Mozilla/5.0 (Windows; U; Windows NT 5.2) Gecko/2008070208 Firefox/3.0.1"
    }
resmap={"1":0,"2":0,"3":0,"4":0,"5":0,"6":0,"7":0,"8":0,"9":0,"10":0,"11":0,"12":0}
for i in range(100):
  res = requests.post(url=url,data=data,headers=headers)
  resmap[res.text]+=1
  time.sleep(0.01)

for i in range(1,13):
  print(str(i)+" "+str(resmap[str(i)]))
