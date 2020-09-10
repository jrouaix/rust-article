import random
from locust import HttpUser, task, between

class QuickstartUser(HttpUser):
    wait_time = between(5, 9)

    @task(1000)
    def uniques(self):
        self.client.get("/uniques/")