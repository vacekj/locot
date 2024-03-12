# Locot

A simple utility to get Lines Of Code Over Time for a git repository. Outputs CSV for visualization and analysis

[Example graph of the Noir repo](https://docs.google.com/spreadsheets/d/1Zx_O20KeL6PlNVs9AYsVBMJqX0vWL0g9I2fP_azxTSk/edit)

## Install

```bash
cargo install locot
```

## Usage
Count the repository in the current directory and redirect the output to a csv file for later analysis

```bash
locot > results.csv
```

### Example output of the locot repository:

| Commit                                   | Time                 | Total | Markdown | Rust | TOML |
|------------------------------------------|----------------------|-------|----------|------|------|
| b9d3f2b3ecc975870cdf9efc714ed441c81caed2 | 2023-05-11T14:11:12Z | 2     | 2        | 0    | 0    |
| 88ab06991a322fe6a731700c255b1a0b0ba5bddb | 2023-05-11T14:15:32Z | 7     | 7        | 0    | 0    |
| fa8e52ae130e1e47e5ec71ff84ec9b3f7d3aa48f | 2023-05-17T12:58:42Z | 76    | 2        | 63   | 11   |
| eb430bb6e3006974e42b67e8603fce10a6c10812 | 2023-05-17T13:02:03Z | 86    | 2        | 73   | 11   |
| 46c56a4a92310242ad17d245db4932f00f726e4a | 2023-05-17T13:03:08Z | 83    | 2        | 70   | 11   |
| ee901b3baf0ec55846ed52c122bcb661a4c01a46 | 2023-05-17T13:05:28Z | 83    | 2        | 70   | 11   |
| 74f6671143de9ba2f398aa6904f549183e0d11cd | 2023-05-17T13:08:18Z | 95    | 2        | 80   | 13   |
| eb7ac70fb50e64d541b923ee31adbd13e2849945 | 2023-07-12T16:53:25Z | 50    | 2        | 34   | 14   |
| 86d4a8e6c32b9993f33216913d56fc88d026a28b | 2023-07-12T17:11:32Z | 72    | 2        | 56   | 14   |
| 87798db277df319d17f8d9df2aa8081d32488cf8 | 2023-07-12T17:13:43Z | 72    | 2        | 56   | 14   |
| 33c9f84069cf805568d5b0a22a1b018552629728 | 2023-07-12T17:35:35Z | 85    | 2        | 68   | 15   |
| d817a9179819a8e3ab9a9e750758be1c1cb931da | 2023-07-12T17:36:41Z | 91    | 8        | 68   | 15   |
| cf15340d9c9f6fd5d841650104d6a867a29e6c95 | 2024-03-11T16:27:33Z | 107   | 8        | 83   | 16   |
| a49da42ed4dae14640085e5fc2e1eca0782633de | 2024-03-12T15:16:52Z | 152   | 8        | 127  | 17   |
| 2fd3ecc89a60a8960a706c875a028e661a966d6d | 2024-03-12T15:26:55Z | 165   | 8        | 139  | 18   |
| 330c9d560fb1ecf005250d5d98fc9c681876cb81 | 2024-03-12T15:28:21Z | 165   | 8        | 139  | 18   |
| 2f60f6b9de2e3575bcd45c47fa6accd26e67dfcf | 2024-03-12T15:29:04Z | 165   | 8        | 139  | 18   |



# License

GNU GPL v3