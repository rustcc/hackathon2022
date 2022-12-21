import axios from 'axios'

export async function uploadFile(file, url) {
  const formData = new FormData()
  formData.append('title', file.file.name)
  formData.append('file', file.file)

  const baseUrl = 'https://api.pinata.cloud'
  url = `${baseUrl}/pinning/pinFileToIPFS`
  file.status = 'loading'
  const rz = await axios.post(
    url,
    formData,
    {
      withCredentials: true,
      maxContentLength: 'Infinity', // this is needed to prevent axios from erroring out with large files
      maxBodyLength: 'Infinity',
      headers: {
        'Content-type': `multipart/form-data; boundary= ${formData._boundary}`,
        // 'Content-Type': 'multipart/form-data',
        'pinata_api_key': PINATA_KEY,
        'pinata_secret_api_key': PINATA_SEC,
        'path': file.file.name,
      },
    })
  file.status = rz.ok

  console.log('====> rz :', rz)
  return rz
  // set up the request data

  // track status and upload file
  // file.status = 'loading'
  // const response = await fetch(url, { method: 'POST', body: formData })

  // // change status to indicate the success of the upload request
  // file.status = response.ok

  // return response
}

export function uploadFiles(files, url) {
  files = unref(files)
  return Promise.all(files.map(file => uploadFile(file, url)))
}

export default function createUploader(url) {
  return {
    uploadFile(file) {
      return uploadFile(file, url)
    },
    uploadFiles(files) {
      return uploadFiles(files, url)
    },
  }
}
