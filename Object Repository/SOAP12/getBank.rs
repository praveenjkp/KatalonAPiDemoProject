<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>getBank</name>
   <tag></tag>
   <elementGuidId>3b815724-850b-4432-9690-7dc12f98dd8d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <restRequestMethod></restRequestMethod>
   <restUrl></restUrl>
   <serviceType>SOAP</serviceType>
   <soapBody>&lt;?xml version=&quot;1.0&quot; encoding=&quot;UTF-8&quot;?>&lt;SOAP-ENV:Envelope xmlns:SOAP-ENV=&quot;http://schemas.xmlsoap.org/soap/envelope/&quot; xmlns:tns=&quot;http://thomas-bayer.com/blz/&quot;>
  &lt;SOAP-ENV:Header/>
  &lt;SOAP-ENV:Body>
    &lt;tns:getBank/>
  &lt;/SOAP-ENV:Body>
&lt;/SOAP-ENV:Envelope>
</soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod>SOAP12</soapRequestMethod>
   <soapServiceFunction>getBank</soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()
</verificationScript>
   <wsdlAddress>http://www.thomas-bayer.com/axis2/services/BLZService?wsdl</wsdlAddress>
</WebServiceRequestEntity>
