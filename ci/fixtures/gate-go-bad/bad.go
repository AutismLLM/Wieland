package gatebad

import (
	"fmt"
	"net/http"
	"os"
	"strconv"
)

func Save(path string, data []byte) {
	f, _ := os.Create(path)
	defer f.Close()
	f.Write(data)
}

func Port(raw string) (int, error) {
	port, err := strconv.Atoi(raw)
	if err != nil {
		return 0, nil
	}
	return port, nil
}

func Read(path string) ([]byte, error) {
	data, err := os.ReadFile(path)
	if err != nil {
		return nil, fmt.Errorf("read %s: %v", path, err)
	}
	return data, nil
}

func Name(v any) string {
	return v.(string)
}

func Status(url string) (int, error) {
	resp, err := http.Get(url)
	if err != nil {
		return 0, err
	}
	return resp.StatusCode, nil
}
