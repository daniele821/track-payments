package payments

import (
	"encoding/json"
	"os"
)

func ConvertFromJsonData(input allPaymentsJson) (AllPayments, error) {
	output := NewAllPayments()
	outputEmpty := NewAllPayments()
	if err := output.AddCities(input.ValueSet.Cities...); err != nil {
		return outputEmpty, err
	}
	if err := output.AddShops(input.ValueSet.Shops...); err != nil {
		return outputEmpty, err
	}
	if err := output.AddPaymentMethods(input.ValueSet.PaymentMethods...); err != nil {
		return outputEmpty, err
	}
	if err := output.AddItems(input.ValueSet.Items...); err != nil {
		return outputEmpty, err
	}
	for _, payment := range input.Payments {
		date := payment.Date
		if err := output.AddPayment(payment.City, payment.Shop, payment.PaymentMethod, date); err != nil {
			return outputEmpty, err
		}
		for _, order := range payment.Orders {
			if err := output.AddOrder(order.Quantity, order.UnitPrice, order.Item, date); err != nil {
				return outputEmpty, err
			}
		}
	}
	return output, nil
}
func NewAllPaymentsFromJson(input string) (AllPayments, error) {
	data := allPaymentsJson{}
	outputEmpty := NewAllPayments()
	if err := json.Unmarshal([]byte(input), &data); err != nil {
		return outputEmpty, err
	}
	output, err := ConvertFromJsonData(data)
	if err != nil {
		return outputEmpty, err
	}
	return output, nil
}
func NewAllPaymentsFromjsonFile(jsonPath string) (AllPayments, error) {
	allPaymentsEmpty := NewAllPayments()
	jsonDataByte, err := os.ReadFile(jsonPath)
	if err != nil {
		fileCreated, err := os.Create(jsonPath)
		if err != nil {
			return allPaymentsEmpty, err
		}
		defer fileCreated.Close()
		if _, err := fileCreated.WriteString("{}"); err != nil {
			return allPaymentsEmpty, err
		}
		jsonDataByte, err = os.ReadFile(jsonPath)
		if err != nil {
			return allPaymentsEmpty, err
		}
	}
	JsonData := string(jsonDataByte)
	// load all payments from json file
	allPayments, err := NewAllPaymentsFromJson(JsonData)
	if err != nil {
		return allPaymentsEmpty, err
	}
	return allPayments, nil
}
