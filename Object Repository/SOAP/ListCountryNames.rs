<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>ListCountryNames</name>
   <tag></tag>
   <elementGuidId>d5f7b249-2750-4e5d-a111-b5f050ff64eb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;Envelope xmlns=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot;>
    &lt;Body>
        &lt;ListOfCountryNamesByName xmlns=&quot;http://www.oorsprong.org/websamples.countryinfo&quot;/>
    &lt;/Body>
&lt;/Envelope></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP</soapRequestMethod>
   <soapServiceFunction>ListOfCountryNamesByName</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()

//WS.verifyElementText(response, 'ListOfCountryNamesByNameResponse.ListOfCountryNamesByNameResult.tCountryCodeAndName[70].sISOCode', 'DK')
//WS.verifyElementText(response, 'ListOfCountryNamesByNameResponse.ListOfCountryNamesByNameResult.tCountryCodeAndName[&quot;Australia&quot;].sISOCode', 'AU')
WS.verifyElementText(response, 'ListOfCountryNamesByNameResponse.ListOfCountryNamesByNameResult.tCountryCodeAndName[2].sISOCode', 'AL')</verificationScript>
   <wsdlAddress>http://www.oorsprong.org/websamples.countryinfo/CountryInfoService.wso?WSDL</wsdlAddress>
</WebServiceRequestEntity>
