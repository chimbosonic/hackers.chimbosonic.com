import json
import os
import uuid

def list_files(directory) -> list:
    list_of_files = []
    for filename in os.listdir(directory):
        if os.path.isfile(os.path.join(directory, filename)):

            list_of_files.append({
                "file_name": filename,
                "id": generate_uuid_from_string(filename),
                "tags": os.path.splitext(filename)[0].split("-")
            })

    return {"gifs": list_of_files}

def generate_uuid_from_string(string):
    return str(uuid.uuid3(uuid.NAMESPACE_URL, string))

def write_to_json_file(data, filename):
    with open(filename, 'w') as outfile:
        json.dump(data, outfile, indent=4)


def main():
    list_of_files = list_files("./gifs/")
    write_to_json_file(list_of_files, "db.json")

main()
