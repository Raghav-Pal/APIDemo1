import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import internal.GlobalVariable as GlobalVariable

response1 = WS.sendRequest(findTestObject('SOAP/ListCountryNames'))

String xml1 = response1.getResponseBodyContent()

def value = new XmlSlurper().parseText(xml1)

def countryCode = value.ListOfCountryNamesByNameResult.tCountryCodeAndName[2].sISOCode.text()

println(' \n... Extracted country code is : ' + countryCode)

GlobalVariable.CountryISOCode = countryCode

WS.sendRequestAndVerify(findTestObject('SOAP/GetCapital', [('countryISOCode') : GlobalVariable.CountryISOCode]))

