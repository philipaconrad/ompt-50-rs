# Python script to clean up `bindgen`-generated OMPT bindings.
# Copyright (c) 2018, Philip Conrad. All rights reserved.
import sys
import string


prefix_map_types = {
  "ompt_callbacks": "callback",
  "ompt_callback": "callback",
  "ompt_scope": "",
  "ompt_set_callback": "set_callback",
  "ompt_sync_region": "sync_region_kind",
  "ompt_": "",
}

# Suffixes to attach
suffix_map_types = {
  "ompt_callbacks": "",
  "ompt_callback": "_fn",
  "ompt_interface": "_fn",
  "ompt_initialize": "_fn",
  "ompt_finalize": "_fn",
  "ompt_set_callback": "_fn",
  "ompt_get_callback": "_fn",
  "ompt_function_lookup": "_fn",
}

prefix_map_other = {
  "ompt_callback": "",
  "ompt_cancel": "",
  "ompt_mutex": "",
  "ompt_scope": "",
  "ompt_set_callback": "",
  "ompt_set": "",
  "omp_state": "",
  "ompt_sync_region": "",
  "ompt_task_dependence": "",
  "ompt_task": "",
  "ompt_work": "",
  "ompt_": "",
}


if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Error: Need refactorings TSV file.")
        exit(1)

    # Load up refactorings lists.
    lines = open(sys.argv[1], 'r').readlines()
    lines = [line.strip() for line in lines]      # Strip out newline chars.
    pairs = [line.split('\t') for line in lines]  # Split by tab delimiter.

    for pair in pairs:
        old, new = pair
        out = new
        # Special cases handled first.
        if out == "ompt_id_none":
            out = "ID_NONE"
            print("{}\t{}".format(old, out))
            continue
        elif out == "omp_wait_id_none":
            out = "OMP_WAIT_ID_NONE"
            print("{}\t{}".format(old, out))
            continue
        # Handle type signatures.
        if new.endswith("_t"):
            out = out[:-2]
            # Attach suffix if needed.
            for prefix in suffix_map_types:
                if out.startswith(prefix):
                    out = out + suffix_map_types[prefix]
                    break
            # Check prefix to see what to do.
            for prefix in prefix_map_types:
                if out.startswith(prefix) and len(out) > len(prefix):
                    new_prefix = prefix_map_types[prefix]
                    out = new_prefix + out[len(prefix):]
                    break
            # Capitalize each word, then strip out underscores.
            out = string.capwords(out, '_').replace('_', '')
        # Handle enums and everything else.
        else:
            for prefix in prefix_map_other:
                if out.startswith(prefix) and len(out) > len(prefix):
                    # Normal case.
                    new_prefix = prefix_map_other[prefix]
                    out = new_prefix + out[len(prefix):]
                    break
            # Capitalize each word, then strip out underscores.
            out = string.capwords(out, '_').replace('_', '')
        print("{}\t{}".format(old, out))
