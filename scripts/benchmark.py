import os
import requests
import time
import random
import string

base_url = 'http://127.0.0.1:8000/v1/'

def generate_random_name(length=8):
    letters = string.ascii_lowercase
    return ''.join(random.choice(letters) for _ in range(length))

def send_request(method, endpoint, data=None):
    url = base_url + endpoint
    start_time = time.time()
    response = None
    
    if method == 'GET':
        response = requests.get(url)
    elif method == 'POST':
        response = requests.post(url, json=data)
    elif method == 'DELETE':
        response = requests.delete(url)
    elif method == 'PUT':
        response = requests.put(url, json=data)
    
    end_time = time.time()
    elapsed_time = end_time - start_time
    
    return response, elapsed_time

def evaluate_request_performance(method, endpoint, data=None):
    response, elapsed_time = send_request(method, endpoint, data)
    
    if response is not None:
        print(f'Response code: {response.status_code}')
        print(f'Response body: {response.text}')
        print(f'Elapsed time: {elapsed_time} seconds')
    else:
        print('Request failed')

def remove_catalog_file():
    file_path = '/Users/yenjuw/CMU/Spring24/15721/15721-s24-catalog2/database/catalog.namespace'
    if os.path.exists(file_path):
        os.remove(file_path)
        print('Removed catalog file')
    else:
        print('Catalog file does not exist')

def main():
    # Remove catalog file
    remove_catalog_file()

    namespace_name = generate_random_name()
    namespaces_endpoint = 'namespaces'
    namespace_endpoint = namespaces_endpoint + '/' + namespace_name
    tables_endpoint = namespace_endpoint + '/tables'

    table_name = generate_random_name()
    table_endpoint = tables_endpoint + '/' + table_name

    renamed_table_name = generate_random_name()
    renamed_table_endpoint = tables_endpoint + '/' + renamed_table_name

    # Create namespace
    evaluate_request_performance('POST', 'namespaces', data={"namespace": [namespace_name]})
    
    # Get namespace
    evaluate_request_performance('GET', namespace_endpoint)
    
    # Create table
    evaluate_request_performance('POST', tables_endpoint, data={"name": table_name})
    
    # List table
    evaluate_request_performance('GET', table_endpoint)
    
    # Rename table
    evaluate_request_performance('POST', 'tables/rename', data={
        "source": { "namespace": [namespace_name], "name": table_name },
        "destination": { "namespace": [namespace_name], "name": renamed_table_name }
    })
    
    # Get renamed table
    evaluate_request_performance('GET', renamed_table_endpoint)
    
    # Delete table
    evaluate_request_performance('DELETE', renamed_table_endpoint)

if __name__ == '__main__':
    main()